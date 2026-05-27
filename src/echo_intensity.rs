/// Parses the Echo Intensity block (ID 0x0300).
///
/// Four beams per depth bin, each a u8 count.
/// Returns the mean of all four beams as a single f32 per bin.

use crate::error::{ParseError, Result};

pub struct EchoIntensityBlock {
    /// Mean intensity across 4 beams for each bin, in raw counts (0–255).
    pub mean_intensity: Vec<f32>,
}

impl EchoIntensityBlock {
    pub fn parse(block: &[u8], num_cells: u8) -> Result<EchoIntensityBlock> {
        let expected = 2 + 4 * num_cells as usize;
        if block.len() < expected {
            return Err(ParseError::Fatal(format!(
                "echo intensity block too short: {} < {}",
                block.len(),
                expected
            )));
        }
        let id = u16::from_le_bytes([block[0], block[1]]);
        if id != 0x0300 {
            return Err(ParseError::Fatal(format!(
                "expected echo intensity ID 0x0300, got 0x{:04X}",
                id
            )));
        }

        let mut mean_intensity = Vec::with_capacity(num_cells as usize);
        for i in 0..num_cells as usize {
            let base = 2 + i * 4;
            let b1 = block[base] as f32;
            let b2 = block[base + 1] as f32;
            let b3 = block[base + 2] as f32;
            let b4 = block[base + 3] as f32;
            mean_intensity.push((b1 + b2 + b3 + b4) / 4.0);
        }

        Ok(EchoIntensityBlock { mean_intensity })
    }
}
