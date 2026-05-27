# src/parser.rs

## Purpose
Top-level ensemble loop. Iterates through the raw file byte slice, locating and
validating each ensemble, then dispatching individual blocks to their typed parsers.

## Data Flow
```
&[u8] (full file)
  → scan for 0x7F 0x7F boundaries
  → for each ensemble:
      validate_checksum()       // fatal on mismatch
      Header::parse()           // read offset table
      FixedLeader::parse()      // geometry constants
      VariableLeader::parse()   // timestamp + depth
      VelocityBlock::parse()    // u/v/w per bin
      EchoIntensityBlock::parse()
      VmDasNav::parse()         // GPS lat/lon
      → coords::to_cartesian()  // GPS → X/Z meters
      → coords::bin_depth_m()   // per-bin Y
      → assemble PingJson
```

## Missing-data handling
If `VelocityBin.u_mm_s == -32768` the bin is `None`. The loop increments
`dropped_corrupt_bins` and skips that bin. Pings with no valid bins are dropped entirely.
Pings missing a VmDas nav block are silently skipped (cannot compute position).

## Error contract
Any unrecoverable parse error propagates as `ParseError::Fatal` up to `lib.rs`.
