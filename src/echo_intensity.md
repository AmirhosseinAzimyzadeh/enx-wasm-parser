# src/echo_intensity.rs

## Purpose
Parses the Echo Intensity block (ID `0x0300`). Four u8 beam counts per depth bin.
Returns the arithmetic mean of all four beams as a single f32 per bin.

## Binary layout
Block size = 2 + 4 × num_cells bytes.
Per bin (4 bytes): beam1 u8, beam2 u8, beam3 u8, beam4 u8.

## Output
`EchoIntensityBlock { mean_intensity: Vec<f32> }` — one value per bin, range 0–255.

## Error contract
Fatal if block is shorter than expected or if ID ≠ `0x0300`.
