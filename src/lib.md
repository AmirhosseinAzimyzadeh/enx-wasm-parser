# src/lib.rs

## Purpose
`wasm-bindgen` entry point. Declares all Rust sub-modules and exports the single public
function `parse_enx` to JavaScript.

## Public API
```rust
fn parse_enx(data: &[u8], anchor_lat: f64, anchor_lon: f64, output_format: &str)
  -> Result<JsValue, JsValue>
```

## Data Flow
```
JS ArrayBuffer (Uint8Array view)
  → parse_enx()
      → parser::parse_file()      // full ensemble loop
          → output::to_binary()   // if output_format == "binary"
          → output::to_json()     // if output_format == "json"
      → assemble JsValue { data, metadata }
  → returned to JS caller
```

## Error contract
Any `ParseError::Fatal` from the parser is converted to a JS `Error` string and returned
as the `Err` variant of the `Result`, which `wasm-bindgen` converts to a rejected Promise.
