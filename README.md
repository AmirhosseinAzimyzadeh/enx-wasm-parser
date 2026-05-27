# enx-wasm-parser

A Rust/WebAssembly npm package that parses **Teledyne RDI PD0/ENX** binary acoustic Doppler current profiler (ADCP) files entirely off the main thread, converts GPS coordinates to local 3D Cartesian space at 64-bit precision, and exposes a clean TypeScript API.

---

## Features

- **Pure Wasm core** — all binary parsing happens in Rust compiled to WebAssembly; no native Node addons, no server round-trips
- **Off main thread** — runs inside a Web Worker automatically; the UI never blocks
- **Zero-copy transfers** — the input `ArrayBuffer` is transferred (not copied) to the worker; the binary output `Float32Array` is transferred back
- **Strict error handling** — any malformed ensemble (bad magic bytes or checksum mismatch) throws immediately; no silent data corruption
- **Missing-data aware** — Teledyne's `-32768` sentinel is detected per bin; corrupt bins are dropped and counted in metadata
- **Two output modes** — structured JSON for inspection/debugging, or a flat interleaved `Float32Array` ready for WebGL upload
- **f64 geo-precision** — GPS→Cartesian conversion is done in 64-bit Rust before downcasting to f32 for output

---

## Supported File Types

| Extension | Description |
|---|---|
| `.enx` | VmDas ensemble file (includes GPS nav block) |
| `.ens` | VmDas ensemble file |
| `.sta` | VmDas short-term average |
| `.lta` | VmDas long-term average |
| `.pd0` | Raw WorkHorse PD0 binary (GPS nav block may be absent) |

> **Note:** Raw `.enr` files do not contain the VmDas navigation block (`0x2000`). Pings without GPS data are skipped — they cannot be placed in Cartesian space.

---

## Output Format

### Binary mode (`outputFormat: 'binary'`)

Returns a `Float32Array` with **7 floats per valid depth bin**, across all pings, interleaved sequentially:

```
[ X, Y, Z, U, V, W, Intensity,   X, Y, Z, U, V, W, Intensity, ... ]
```

| Field | Unit | Description |
|---|---|---|
| X | meters | East offset from anchor |
| Y | meters | Negative depth (−5 m = 5 m below surface) |
| Z | meters | North offset from anchor |
| U | m/s | Eastward velocity |
| V | m/s | Northward velocity |
| W | m/s | Vertical velocity |
| Intensity | counts (0–255) | Mean echo intensity across 4 beams |

### JSON mode (`outputFormat: 'json'`)

Returns a serialized JSON string of the form:

```json
[
  {
    "timestamp": "2023-07-14T12:34:56.00Z",
    "x": 12.3,
    "z": -45.6,
    "bins": [
      { "y": -5.0, "u": 0.123, "v": -0.045, "w": 0.002, "intensity": 87.25 }
    ]
  }
]
```

---

## Installation

```bash
npm install enx-wasm-parser
```

Or clone and build from source (see [Building from Source](#building-from-source)).

---

## Usage

```typescript
import { ENXParser } from 'enx-wasm-parser';

const parser = new ENXParser();

// Get an ArrayBuffer from a file input, fetch, or fs.readFile
const fileBuffer: ArrayBuffer = /* ... */;

const result = await parser.parse(fileBuffer, {
  outputFormat: 'binary',   // 'binary' | 'json'  (default: 'json')
  anchor: {
    lat: 45.27,             // WGS-84 decimal degrees
    lon: -66.05,
  },
});

// Binary mode
const floats = result.data as Float32Array;
const totalBins = floats.length / 7;
console.log(`${totalBins} valid depth bins`);

// JSON mode
const pings = JSON.parse(result.data as string);
console.log(pings[0]);

// Metadata
console.log(result.metadata);
// {
//   totalPings: 1200,
//   validPings: 1198,
//   droppedCorruptBins: 14,
//   pingsWithoutGps: 0,    // >0 for raw PD0 files (no GPS block)
//   durationMs: 42.3
// }

// Release the worker when done
parser.dispose();
```

### File input example (browser)

```typescript
document.querySelector('input[type=file]').addEventListener('change', async (e) => {
  const file = (e.target as HTMLInputElement).files![0];
  const buffer = await file.arrayBuffer();

  const parser = new ENXParser();
  const result = await parser.parse(buffer, {
    outputFormat: 'binary',
    anchor: { lat: 45.27, lon: -66.05 },
  });

  console.log(result.metadata);
});
```

> **Important:** `ArrayBuffer` is transferred zero-copy to the worker. After calling `parse()`, the original `buffer` reference becomes detached (neutered). If you need to re-read the bytes, clone the buffer before passing it: `buffer.slice(0)`.

---

## API Reference

### `new ENXParser()`

Creates a new parser instance. The Web Worker is not spawned until the first `parse()` call.

---

### `parser.parse(buffer, options): Promise<ParseResult>`

Parses the binary file and returns a promise that resolves with the result.

**Parameters:**

| Name | Type | Required | Description |
|---|---|---|---|
| `buffer` | `ArrayBuffer` | Yes | The raw ENX/PD0 file bytes |
| `options.anchor` | `{ lat: number, lon: number }` | Yes | Geographic anchor point for Cartesian conversion |
| `options.outputFormat` | `'json' \| 'binary'` | No | Output format (default: `'json'`) |

**Returns:** `Promise<ParseResult>`

```typescript
interface ParseResult {
  data: Float32Array | string;  // Float32Array for binary, JSON string for json
  metadata: {
    totalPings: number;          // all ensembles found in the file
    validPings: number;          // ensembles with all required acoustic blocks present
    droppedCorruptBins: number;  // bins dropped due to -32768 missing-data flag
    pingsWithoutGps: number;     // pings with no VmDas GPS block (raw PD0 files); x/z = null
    durationMs: number;          // parse wall-clock time in milliseconds
  };
}
```

**Throws:** If the file fails magic-byte or checksum validation, the promise rejects with an `Error` describing the failure location.

---

### `parser.dispose()`

Terminates the background Web Worker and rejects any pending `parse()` promises. Call this when the parser is no longer needed to free memory.

---

## Building from Source

### Prerequisites

| Tool | Install |
|---|---|
| Rust + Cargo | https://rustup.rs |
| wasm-pack | `cargo install wasm-pack` |
| Node.js ≥ 18 | https://nodejs.org |

### Steps

```bash
# 1. Clone
git clone https://github.com/your-org/enx-wasm-parser
cd enx-wasm-parser

# 2. Build the Wasm module
npm run build:wasm
# Outputs to pkg/

# 3. Install JS dependencies
npm install

# 4. Type-check the TypeScript
npx tsc --noEmit

# 5. Run the example dev server
npm run dev
# → http://localhost:5173
```

### Build scripts

| Script | What it does |
|---|---|
| `npm run build:wasm` | Runs `wasm-pack build --target bundler --out-dir pkg` |
| `npm run build:ts` | Compiles TypeScript to `dist/` |
| `npm run build` | Runs both of the above in sequence |
| `npm run dev` | Starts the Vite dev server for the example app |

---

## Project Structure

```
enx-wasm-parser/
├── src/                        # Rust source
│   ├── lib.rs / lib.md         # wasm-bindgen entry point
│   ├── parser.rs / parser.md   # ensemble loop + checksum
│   ├── header.rs / header.md   # 0x7F 0x7F magic + offset table
│   ├── fixed_leader.rs         # block 0x0000 — geometry constants
│   ├── variable_leader.rs      # block 0x0080 — timestamp + depth
│   ├── velocity.rs             # block 0x0100 — u/v/w per bin
│   ├── echo_intensity.rs       # block 0x0300 — signal strength
│   ├── vmdas_nav.rs            # block 0x2000 — GPS lat/lon
│   ├── coords.rs               # GPS→Cartesian (f64) + bin depth
│   ├── output.rs               # JSON / Float32Array serialization
│   └── error.rs                # ParseError type
├── ts/                         # TypeScript source
│   ├── index.ts / index.md     # ENXParser public API class
│   ├── worker.ts / worker.md   # Web Worker entry point
│   └── types.ts                # Shared interfaces
├── example/                    # Vite demo app
│   ├── index.html
│   ├── main.ts
│   └── vite.config.ts
├── pkg/                        # wasm-pack output (gitignored)
├── Cargo.toml
├── package.json
├── tsconfig.json
├── vite.config.ts
└── RESEARCH.md                 # Binary format spec + design decisions
```

Every Rust and TypeScript module has a companion `.md` file documenting its exact purpose, binary layout, and data flow.

---

## How It Works

### Parse pipeline

```
File ArrayBuffer
  └─► Web Worker (off main thread)
        └─► Rust/Wasm: parse_enx()
              ├─ scan for 0x7F 0x7F ensemble boundaries
              ├─ validate 16-bit checksum per ensemble
              ├─ read Header → offset table
              ├─ parse Fixed Leader  → num_cells, cell geometry
              ├─ parse Variable Leader → timestamp, transducer depth
              ├─ parse Velocity block  → u/v/w per bin (drop -32768 bins)
              ├─ parse Echo Intensity  → mean of 4 beams per bin
              ├─ parse VmDas Nav       → GPS lat/lon (i32 × 1e-7)
              ├─ GPS → Cartesian (f64 equirectangular)
              └─ serialize → Float32Array or JSON
        └─► transfer result back to main thread
  └─► Promise resolves
```

### Coordinate system

The anchor point (user-supplied lat/lon) becomes the origin `(0, 0, 0)`.

- **X** — East, meters
- **Y** — negative depth, meters (`Y = −5` means 5 m below surface)
- **Z** — North, meters

Conversion uses the equirectangular approximation with midpoint-latitude correction, accurate to sub-meter for survey distances under ~500 km.

### Missing-data handling

Teledyne RDI uses the value `-32768` as a "no valid data" sentinel for the East velocity component. When the parser encounters this value in a bin:

1. The entire bin (all four velocity components + intensity) is discarded
2. `droppedCorruptBins` in the metadata is incremented
3. Surrounding bins in the same ping are unaffected

---

## Error Handling

The parser is intentionally strict:

| Condition | Result |
|---|---|
| File does not start with `0x7F 0x7F` | Promise rejects with fatal error |
| Ensemble checksum mismatch | Promise rejects with fatal error |
| Required block missing or malformed | Promise rejects with fatal error |
| Velocity bin = `-32768` (missing data) | Bin silently dropped, counter incremented |
| VmDas nav block absent in an ensemble | That ping is silently skipped (no GPS) |

---

## License

MIT
