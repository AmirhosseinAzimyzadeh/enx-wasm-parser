/// Parses the VmDas Navigation block (ID 0x2000) — 92 bytes.
///
/// Extracts high-precision GPS latitude and longitude (i32 × 1e-7 degrees).
/// Block is only present in ENX/ENS/STA/LTA files. Ping is skipped if absent.
///
/// Two offset conventions exist in the wild; we validate whichever pair
/// produces geographically sane values (lat ∈ [-90,90], lon ∈ [-180,180]).

use crate::error::{ParseError, Result};

pub struct VmDasNav {
    /// Decimal degrees, WGS-84.
    pub latitude: f64,
    /// Decimal degrees, WGS-84.
    pub longitude: f64,
}

impl VmDasNav {
    pub fn parse(block: &[u8]) -> Result<VmDasNav> {
        if block.len() < 46 {
            return Err(ParseError::Fatal(format!(
                "VmDas nav block too short: {} bytes",
                block.len()
            )));
        }
        let id = u16::from_le_bytes([block[0], block[1]]);
        if id != 0x2000 {
            return Err(ParseError::Fatal(format!(
                "expected VmDas nav ID 0x2000, got 0x{:04X}",
                id
            )));
        }

        // Primary layout: lon at bytes 18-21, lat at bytes 22-25
        let lon_primary = i32::from_le_bytes([block[18], block[19], block[20], block[21]]) as f64 / 1e7;
        let lat_primary = i32::from_le_bytes([block[22], block[23], block[24], block[25]]) as f64 / 1e7;

        if is_valid_lat(lat_primary) && is_valid_lon(lon_primary) {
            return Ok(VmDasNav { latitude: lat_primary, longitude: lon_primary });
        }

        // Alternate layout seen in some VmDas firmware: lon at 10-13, lat at 14-17
        if block.len() >= 18 {
            let lon_alt = i32::from_le_bytes([block[10], block[11], block[12], block[13]]) as f64 / 1e7;
            let lat_alt = i32::from_le_bytes([block[14], block[15], block[16], block[17]]) as f64 / 1e7;
            if is_valid_lat(lat_alt) && is_valid_lon(lon_alt) {
                return Ok(VmDasNav { latitude: lat_alt, longitude: lon_alt });
            }
        }

        Err(ParseError::Fatal(format!(
            "VmDas nav block contains no valid GPS fix (lat={}, lon={})",
            lat_primary, lon_primary
        )))
    }
}

#[inline]
fn is_valid_lat(v: f64) -> bool {
    v >= -90.0 && v <= 90.0 && v != 0.0
}

#[inline]
fn is_valid_lon(v: f64) -> bool {
    v >= -180.0 && v <= 180.0 && v != 0.0
}
