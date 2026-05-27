# src/header.rs

## Purpose
Parses the PD0 ensemble header (bytes 0–5 + 2×N offset entries).

## Binary layout
| Bytes | Field |
|---|---|
| 0–1 | Magic `0x7F 0x7F` |
| 2–3 | `bytes_in_ensemble` (u16 LE) |
| 4 | Reserved |
| 5 | `num_data_types` (u8) |
| 6 + 2n | Offset to block n (u16 LE) |

## Output
`Header { bytes_in_ensemble, offsets: Vec<(block_id, byte_offset)> }`

The `block_id` is read from the first two bytes at each `byte_offset` so the parser
can dispatch without knowing block order in advance.

## Error contract
Fatal if magic bytes are wrong or if an offset points outside the ensemble slice.
