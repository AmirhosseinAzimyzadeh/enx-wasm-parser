/// Parses the Variable Leader block (ID 0x0080) — 65 or 77 bytes depending on firmware.
///
/// Extracts the ensemble timestamp and transducer depth (in decimeters).
/// Block size is determined by the header offset table, not hardcoded.

use crate::error::{ParseError, Result};

pub struct VariableLeader {
    /// ISO-8601 timestamp string: "YYYY-MM-DDTHH:MM:SS.ccZ"
    pub timestamp: String,
    /// Transducer depth in decimeters (divide by 10 for meters).
    pub transducer_depth_dm: u16,
}

impl VariableLeader {
    pub fn parse(block: &[u8]) -> Result<VariableLeader> {
        if block.len() < 56 {
            return Err(ParseError::Fatal(format!(
                "variable leader too short: {} bytes",
                block.len()
            )));
        }
        let id = u16::from_le_bytes([block[0], block[1]]);
        if id != 0x0080 {
            return Err(ParseError::Fatal(format!(
                "expected variable leader ID 0x0080, got 0x{:04X}",
                id
            )));
        }

        let year = 2000u16 + block[4] as u16;
        let month = block[5];
        let day = block[6];
        let hour = block[7];
        let minute = block[8];
        let second = block[9];
        let hundredths = block[10];

        let timestamp = format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:02}Z",
            year, month, day, hour, minute, second, hundredths
        );

        let transducer_depth_dm = u16::from_le_bytes([block[16], block[17]]);

        Ok(VariableLeader { timestamp, transducer_depth_dm })
    }
}
