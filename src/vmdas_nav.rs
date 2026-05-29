/// Parses the VmDas Navigation block (ID 0x2000).
///
/// Layout confirmed from dolfyn source (io/rdi.py, `read_vmdas`).
/// All offsets are from block start (include the 2-byte 0x2000 ID prefix).
/// Scaling: i32 × (180 / 2³¹)  — NOT × 1e-7.
///
///  Bytes 00–01: block ID = 0x2000
///  Bytes 02–05: UTC date (4 × u8: year-2000, month, day, ?)
///  Bytes 06–09: UTC time of first GPS fix (u32, ×0.1 ms)
///  Bytes 10–13: PC clock offset from UTC (i32, ms)
///  Bytes 14–17: latitude  – first GPS fix  (i32, × 180/2³¹ degrees)
///  Bytes 18–21: longitude – first GPS fix  (i32, × 180/2³¹ degrees)
///  Bytes 22–25: UTC time of last GPS fix   (u32, ×0.1 ms)
///  Bytes 26–29: latitude  – last GPS fix   (i32, × 180/2³¹ degrees)  ← primary
///  Bytes 30–33: longitude – last GPS fix   (i32, × 180/2³¹ degrees)  ← primary

use crate::error::{ParseError, Result};

/// Degrees = raw_i32 × GPS_SCALE
const GPS_SCALE: f64 = 180.0 / 2_147_483_648.0; // 180 / 2^31

pub struct VmDasNav {
    pub latitude: f64,
    pub longitude: f64,
}

impl VmDasNav {
    pub fn parse(block: &[u8]) -> Result<VmDasNav> {
        if block.len() < 34 {
            return Err(ParseError::Fatal(format!(
                "VmDas nav block too short: {} bytes (need 34)",
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

        // Primary: last GPS fix (what dolfyn exposes as latitude_gps / longitude_gps)
        let lat = read_i32(block, 26) as f64 * GPS_SCALE;
        let lon = read_i32(block, 30) as f64 * GPS_SCALE;

        if is_valid_coord(lat, 90.0) && is_valid_coord(lon, 180.0) {
            return Ok(VmDasNav { latitude: lat, longitude: lon });
        }

        // Fallback: first GPS fix (bytes 14–21)
        let lat1 = read_i32(block, 14) as f64 * GPS_SCALE;
        let lon1 = read_i32(block, 18) as f64 * GPS_SCALE;

        if is_valid_coord(lat1, 90.0) && is_valid_coord(lon1, 180.0) {
            return Ok(VmDasNav { latitude: lat1, longitude: lon1 });
        }

        Err(ParseError::Fatal(format!(
            "VmDas nav block: no valid GPS fix \
             (last: {:.6},{:.6}  first: {:.6},{:.6})",
            lat, lon, lat1, lon1
        )))
    }
}

#[inline]
fn read_i32(block: &[u8], off: usize) -> i32 {
    i32::from_le_bytes([block[off], block[off+1], block[off+2], block[off+3]])
}

#[inline]
fn is_valid_coord(v: f64, limit: f64) -> bool {
    v > -limit && v < limit && v != 0.0
}
