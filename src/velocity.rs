/// Parses the Velocity Profile block (ID 0x0100).
///
/// Each depth bin has four i16 LE velocities (East/U, North/V, Vertical/W, Error).
/// A bin with beam-1 (East) == -32768 is Teledyne's "missing data" sentinel —
/// that bin is dropped and the caller's droppedCorruptBins counter is incremented.

use crate::error::{ParseError, Result};

pub struct VelocityBin {
    /// East velocity in mm/s.
    pub u_mm_s: i16,
    /// North velocity in mm/s.
    pub v_mm_s: i16,
    /// Vertical velocity in mm/s.
    pub w_mm_s: i16,
}

pub struct VelocityBlock {
    /// Valid bins only (missing-data bins have been filtered out).
    pub bins: Vec<Option<VelocityBin>>,
}

impl VelocityBlock {
    /// `num_cells` is taken from the Fixed Leader.
    pub fn parse(block: &[u8], num_cells: u8) -> Result<VelocityBlock> {
        let expected = 2 + 8 * num_cells as usize;
        if block.len() < expected {
            return Err(ParseError::Fatal(format!(
                "velocity block too short: {} < {}",
                block.len(),
                expected
            )));
        }
        let id = u16::from_le_bytes([block[0], block[1]]);
        if id != 0x0100 {
            return Err(ParseError::Fatal(format!(
                "expected velocity ID 0x0100, got 0x{:04X}",
                id
            )));
        }

        let mut bins = Vec::with_capacity(num_cells as usize);
        for i in 0..num_cells as usize {
            let base = 2 + i * 8;
            let u = i16::from_le_bytes([block[base], block[base + 1]]);
            if u == -32768 {
                bins.push(None); // missing data sentinel
                continue;
            }
            let v = i16::from_le_bytes([block[base + 2], block[base + 3]]);
            let w = i16::from_le_bytes([block[base + 4], block[base + 5]]);
            bins.push(Some(VelocityBin { u_mm_s: u, v_mm_s: v, w_mm_s: w }));
        }

        Ok(VelocityBlock { bins })
    }
}
