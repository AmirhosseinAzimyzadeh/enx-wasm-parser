/// Parses the Fixed Leader block (ID 0x0000) — 59 bytes.
///
/// Extracts the three geometry fields needed to compute depth-bin centers:
/// number_of_cells, cell_size_cm, blank_distance_cm.

use crate::error::{ParseError, Result};

pub struct FixedLeader {
    pub num_cells: u8,
    /// Raw cell size in centimeters.
    pub cell_size_cm: u16,
    /// Blank-after-transmit distance in centimeters.
    pub blank_distance_cm: u16,
}

impl FixedLeader {
    pub fn parse(block: &[u8]) -> Result<FixedLeader> {
        if block.len() < 59 {
            return Err(ParseError::Fatal(format!(
                "fixed leader too short: {} bytes",
                block.len()
            )));
        }
        let id = u16::from_le_bytes([block[0], block[1]]);
        if id != 0x0000 {
            return Err(ParseError::Fatal(format!(
                "expected fixed leader ID 0x0000, got 0x{:04X}",
                id
            )));
        }
        Ok(FixedLeader {
            num_cells: block[9],
            cell_size_cm: u16::from_le_bytes([block[12], block[13]]),
            blank_distance_cm: u16::from_le_bytes([block[14], block[15]]),
        })
    }
}
