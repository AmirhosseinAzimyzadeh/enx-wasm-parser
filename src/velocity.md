# src/velocity.rs

## Purpose
Parses the Velocity Profile block (ID `0x0100`). Provides East (U), North (V), and
Vertical (W) velocity components for every depth bin in mm/s.

## Missing-data sentinel
Teledyne RDI uses `-32768` (`0x8000`) as the "no data" flag for beam-1 (East velocity).
Any bin where `u_mm_s == -32768` is returned as `None` in the output Vec. The caller
increments `droppedCorruptBins` and excludes that bin from the final output.

## Binary layout
Block size = 2 + 8 × num_cells bytes.
Per bin (8 bytes): u i16, v i16, w i16, error i16 (all LE).

## Error contract
Fatal if block is shorter than expected or if ID ≠ `0x0100`.
