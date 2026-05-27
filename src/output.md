# src/output.rs

## Purpose
Converts the `Vec<PingJson>` assembled by the parser into one of two output formats:
a JSON string or a flat `Vec<f32>` for zero-copy transfer as a `Float32Array`.

## JSON mode
Serializes `Vec<PingJson>` with `serde_json::to_string`. Each `PingJson` contains:
- `timestamp` — ISO-8601 string
- `x`, `z` — Cartesian offsets in meters (f32)
- `bins[]` — `{ y, u, v, w, intensity }` (all f32)

## Binary mode
Produces an interleaved flat buffer of 7 f32 values per valid bin:
```
[X, Y, Z, U, V, W, Intensity,  X, Y, Z, ...]
```
- X/Z identical for all bins in the same ping
- Y = negative depth (meters)
- U/V/W in m/s (raw mm/s ÷ 1000)
- Intensity = mean of 4 beams

The `Float32Array` is constructed in `lib.rs` from this Vec via `copy_from`, which is
a single memcpy into Wasm linear memory, then transferred back to JS via `Transferable`.
