/// Web Worker: loads the Wasm module once, then handles parse requests.
///
/// Receives a WorkerRequest (ArrayBuffer transferred zero-copy from the main thread),
/// calls the Rust parse_enx function, and posts a WorkerResponse back.

import type { WorkerRequest, WorkerResponse } from './types';

// Dynamic import for the wasm-pack bundler target
let wasmReady: Promise<typeof import('../pkg/enx_wasm_parser')>;

function getWasm() {
  if (!wasmReady) {
    wasmReady = import('../pkg/enx_wasm_parser');
  }
  return wasmReady;
}

self.addEventListener('message', async (event: MessageEvent<WorkerRequest>) => {
  const { id, buffer, options } = event.data;

  try {
    const wasm = await getWasm();
    const bytes = new Uint8Array(buffer);
    const fmt = options.outputFormat ?? 'json';
    const anchorLat = options.anchor.lat;
    const anchorLon = options.anchor.lon;

    const raw = wasm.parse_enx(bytes, anchorLat, anchorLon, fmt);

    const response: WorkerResponse = { id, result: raw as any };

    if (fmt === 'binary' && raw.data instanceof Float32Array) {
      // Transfer the underlying ArrayBuffer back zero-copy
      self.postMessage(response, [raw.data.buffer]);
    } else {
      self.postMessage(response);
    }
  } catch (err: unknown) {
    const response: WorkerResponse = {
      id,
      error: err instanceof Error ? err.message : String(err),
    };
    self.postMessage(response);
  }
});
