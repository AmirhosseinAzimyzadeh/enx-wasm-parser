# src/variable_leader.rs

## Purpose
Parses the Variable Leader block (ID `0x0080`). Contains per-ensemble data:
timestamp and transducer depth. Block size varies (65 or 77 bytes) between firmware
versions; the parser uses the header offset table to locate it rather than fixed sizes.

## Key extracted fields
| Field | Byte(s) | Notes |
|---|---|---|
| timestamp | 4–10 | YY MM DD HH MM SS CC → ISO-8601 string |
| `transducer_depth_dm` | 16–17 | decimeters; divide by 10 for meters |

## Error contract
Fatal if block is shorter than 56 bytes or if ID ≠ `0x0080`.
