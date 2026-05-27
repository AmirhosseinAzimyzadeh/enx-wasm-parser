# src/vmdas_nav.rs

## Purpose
Parses the VmDas Navigation block (ID `0x2000`, 92 bytes). Present only in ENX/ENS/STA/LTA
files produced by Teledyne's VmDas acquisition software. Provides high-precision GPS
latitude and longitude as `i32 × 1e-7` decimal degrees.

## GPS offset ambiguity
Two byte-offset conventions appear across VmDas firmware versions:
- **Primary**: lon at bytes 18–21, lat at bytes 22–25
- **Alternate**: lon at bytes 10–13, lat at bytes 14–17

The parser tries the primary layout first; if the resulting values fall outside
valid geographic ranges (lat ∉ [-90,90] or lon ∉ [-180,180] or == 0), it falls
back to the alternate layout. If neither yields valid coordinates, the parse returns
an error and the calling ensemble is skipped (no GPS = no Cartesian position).

## Error contract
Returns `ParseError::Fatal` if both layouts yield invalid coordinates.
Block absence is handled by the caller — missing nav → ping skipped silently.
