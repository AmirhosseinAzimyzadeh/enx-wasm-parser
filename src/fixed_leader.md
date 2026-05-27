# src/fixed_leader.rs

## Purpose
Parses the Fixed Leader block (ID `0x0000`, 59 bytes). Contains the static instrument
configuration that is constant across all ensembles in a file.

## Key extracted fields
| Field | Byte(s) | Use |
|---|---|---|
| `num_cells` | 9 | loop bound for per-bin parsing |
| `cell_size_cm` | 12–13 | depth-bin center calculation |
| `blank_distance_cm` | 14–15 | depth-bin center calculation |

## Error contract
Fatal if block is shorter than 59 bytes or if ID ≠ `0x0000`.
