/// Parses the PD0 ensemble header block (first 6+ bytes of every ensemble).
///
/// Returns the total ensemble byte-count and a list of (data-type-id, byte-offset) pairs.
/// Validates the 0x7F 0x7F magic bytes; caller is responsible for checksum.

use crate::error::{ParseError, Result};

pub struct Header {
    /// Total bytes in this ensemble **excluding** the 2-byte checksum. Add 2 for the full on-disk size.
    pub bytes_in_ensemble: u16,
    /// (block_id, offset_from_ensemble_start) for every data type in this ensemble.
    pub offsets: Vec<(u16, u16)>,
}

impl Header {
    /// `buf` must start at the beginning of an ensemble (at the 7F 7F magic bytes).
    pub fn parse(buf: &[u8]) -> Result<Header> {
        if buf.len() < 6 {
            return Err(ParseError::Fatal("ensemble too short for header".into()));
        }
        if buf[0] != 0x7F || buf[1] != 0x7F {
            return Err(ParseError::Fatal(
                format!("bad magic bytes: {:02X} {:02X}", buf[0], buf[1]),
            ));
        }

        let bytes_in_ensemble = u16::from_le_bytes([buf[2], buf[3]]);
        // buf[4] is reserved
        let num_types = buf[5] as usize;

        let header_size = 6 + num_types * 2;
        if buf.len() < header_size {
            return Err(ParseError::Fatal("ensemble too short for offset table".into()));
        }

        let mut offsets = Vec::with_capacity(num_types);
        for i in 0..num_types {
            let off_pos = 6 + i * 2;
            let offset = u16::from_le_bytes([buf[off_pos], buf[off_pos + 1]]);
            if (offset as usize) + 2 > buf.len() {
                return Err(ParseError::Fatal(format!(
                    "offset {} for type {} points outside ensemble",
                    offset, i
                )));
            }
            let block_id = u16::from_le_bytes([buf[offset as usize], buf[offset as usize + 1]]);
            offsets.push((block_id, offset));
        }

        Ok(Header { bytes_in_ensemble, offsets })
    }
}
