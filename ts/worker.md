# ts/worker.ts

## Purpose
Web Worker entry point. Loads the Wasm module once (lazy singleton) and handles
`WorkerRequest` messages from the main thread.

## Data flow
```
main thread
  postMessage({ id, buffer, options })  ← ArrayBuffer transferred (zero-copy)
worker
  → import('../pkg/enx_wasm_parser')    ← Wasm loaded once, cached
  → new Uint8Array(buffer)              ← view into transferred memory
  → wasm.parse_enx(bytes, lat, lon, fmt)
  → postMessage({ id, result })
      binary: [result.data.buffer]      ← transfer Float32Array buffer back
      json:   no transfer list
main thread
  Promise resolves with result
```

## Error handling
Any throw from `parse_enx` (Rust `ParseError::Fatal`) is caught, stringified, and
sent back as `{ id, error: string }`. The main thread rejects the corresponding Promise.
