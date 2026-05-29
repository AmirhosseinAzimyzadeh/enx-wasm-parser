/// Top-level ensemble loop and checksum validation.

use crate::coords::{bin_depth_m, to_cartesian};
use crate::echo_intensity::EchoIntensityBlock;
use crate::error::{ParseError, Result};
use crate::fixed_leader::FixedLeader;
use crate::header::Header;
use crate::output::{BinJson, PingJson};
use crate::variable_leader::VariableLeader;
use crate::velocity::VelocityBlock;
use crate::vmdas_nav::VmDasNav;

pub struct ParseResult {
    pub pings: Vec<PingJson>,
    pub total_pings: u32,
    pub dropped_corrupt_bins: u32,
    pub pings_without_gps: u32,
}

pub fn parse_file(
    data: &[u8],
    anchor_lat: f64,
    anchor_lon: f64,
) -> Result<ParseResult> {
    if data.len() < 2 || data[0] != 0x7F || data[1] != 0x7F {
        return Err(ParseError::Fatal("file does not start with 0x7F 0x7F".into()));
    }

    let mut pings = Vec::new();
    let mut total_pings: u32 = 0;
    let mut dropped_corrupt_bins: u32 = 0;
    let mut pings_without_gps: u32 = 0;
    let mut pos = 0usize;

    while pos + 2 <= data.len() {
        if data[pos] != 0x7F || data[pos + 1] != 0x7F {
            pos += 1;
            continue;
        }

        let header = Header::parse(&data[pos..])?;
        let data_len = header.bytes_in_ensemble as usize;
        let ensemble_len = data_len + 2;

        if pos + ensemble_len > data.len() {
            break;
        }

        let ensemble = &data[pos..pos + ensemble_len];
        validate_checksum(ensemble)?;

        total_pings += 1;

        if let Some(ping) = parse_ensemble(
            ensemble,
            &header,
            anchor_lat,
            anchor_lon,
            &mut dropped_corrupt_bins,
            &mut pings_without_gps,
        )? {
            pings.push(ping);
        }

        pos += ensemble_len;
    }

    Ok(ParseResult { pings, total_pings, dropped_corrupt_bins, pings_without_gps })
}

fn validate_checksum(ensemble: &[u8]) -> Result<()> {
    let len = ensemble.len();
    if len < 2 {
        return Err(ParseError::Fatal("ensemble too short for checksum".into()));
    }
    let stored = u16::from_le_bytes([ensemble[len - 2], ensemble[len - 1]]);
    let computed: u16 = ensemble[..len - 2]
        .iter()
        .fold(0u32, |acc, &b| acc + b as u32) as u16;
    if computed != stored {
        return Err(ParseError::Fatal(format!(
            "checksum mismatch: computed 0x{:04X}, stored 0x{:04X}",
            computed, stored
        )));
    }
    Ok(())
}

fn parse_ensemble(
    ensemble: &[u8],
    header: &Header,
    anchor_lat: f64,
    anchor_lon: f64,
    dropped: &mut u32,
    without_gps: &mut u32,
) -> Result<Option<PingJson>> {
    let mut fixed: Option<FixedLeader> = None;
    let mut variable: Option<VariableLeader> = None;
    let mut velocity: Option<VelocityBlock> = None;
    let mut echo: Option<EchoIntensityBlock> = None;
    let mut nav: Option<VmDasNav> = None;

    for &(block_id, offset) in &header.offsets {
        let block = &ensemble[offset as usize..];
        match block_id {
            0x0000 => fixed = Some(FixedLeader::parse(block)?),
            0x0080 => variable = Some(VariableLeader::parse(block)?),
            0x0100 => {
                if let Some(ref fl) = fixed {
                    velocity = Some(VelocityBlock::parse(block, fl.num_cells)?);
                }
            }
            0x0300 => {
                if let Some(ref fl) = fixed {
                    echo = Some(EchoIntensityBlock::parse(block, fl.num_cells)?);
                }
            }
            0x2000 => {
                if let Ok(n) = VmDasNav::parse(block) {
                    nav = Some(n);
                }
            }
            _ => {}
        }
    }

    let (fl, vl, vel, ec) = match (fixed, variable, velocity, echo) {
        (Some(fl), Some(vl), Some(vel), Some(ec)) => (fl, vl, vel, ec),
        _ => return Ok(None),
    };

    let (lat, lon, x, z) = match nav {
        Some(n) => {
            let (cx, cz) = to_cartesian(anchor_lat, anchor_lon, n.latitude, n.longitude);
            (Some(n.latitude), Some(n.longitude), Some(cx as f32), Some(cz as f32))
        }
        None => {
            *without_gps += 1;
            (None, None, None, None)
        }
    };

    let mut bins = Vec::with_capacity(fl.num_cells as usize);
    for i in 0..fl.num_cells as usize {
        match &vel.bins[i] {
            None => { *dropped += 1; }
            Some(bin) => {
                let depth = bin_depth_m(
                    fl.blank_distance_cm,
                    fl.cell_size_cm,
                    i,
                    vl.transducer_depth_dm,
                );
                bins.push(BinJson {
                    y: -(depth as f32),
                    u: bin.u_mm_s as f32 / 1000.0,
                    v: bin.v_mm_s as f32 / 1000.0,
                    w: bin.w_mm_s as f32 / 1000.0,
                    intensity: ec.mean_intensity[i],
                });
            }
        }
    }

    if bins.is_empty() {
        return Ok(None);
    }

    Ok(Some(PingJson { timestamp: vl.timestamp, lat, lon, x, z, bins }))
}
