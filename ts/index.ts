/// ENXParser — public API class for enx-wasm-parser.
///
/// Spawns a Web Worker on first use and keeps it alive for subsequent calls.
/// The ArrayBuffer is transferred to the worker (zero-copy) and the result
/// is returned as a plain Promise — no Worker API leaks to the caller.

import type { ParseOptions, ParseResult, WorkerRequest, WorkerResponse } from './types';

export type { ParseOptions, ParseResult, ParseMetadata } from './types';

let _requestId = 0;

export class ENXParser {
  private _worker: Worker | null = null;
  private _pending = new Map<number, {
    resolve: (v: ParseResult) => void;
    reject: (e: Error) => void;
  }>();

  private _getWorker(): Worker {
    if (!this._worker) {
      // Vite resolves new URL(...) at build time into the worker bundle
      this._worker = new Worker(new URL('./worker.ts', import.meta.url), {
        type: 'module',
      });
      this._worker.addEventListener('message', (event: MessageEvent<WorkerResponse>) => {
        const { id, result, error } = event.data;
        const pending = this._pending.get(id);
        if (!pending) return;
        this._pending.delete(id);
        if (error) {
          pending.reject(new Error(error));
        } else {
          pending.resolve(result!);
        }
      });
      this._worker.addEventListener('error', (event) => {
        // Reject all pending promises on unrecoverable worker error
        const msg = event.message ?? 'Worker crashed';
        for (const p of this._pending.values()) {
          p.reject(new Error(msg));
        }
        this._pending.clear();
        this._worker = null;
      });
    }
    return this._worker;
  }

  parse(buffer: ArrayBuffer, options: ParseOptions): Promise<ParseResult> {
    return new Promise((resolve, reject) => {
      const id = ++_requestId;
      this._pending.set(id, { resolve, reject });
      const request: WorkerRequest = { id, buffer, options };
      // Transfer the buffer — zero-copy handoff to the worker
      this._getWorker().postMessage(request, [buffer]);
    });
  }

  /** Terminate the background worker and free resources. */
  dispose(): void {
    this._worker?.terminate();
    this._worker = null;
    for (const p of this._pending.values()) {
      p.reject(new Error('ENXParser disposed'));
    }
    this._pending.clear();
  }
}
