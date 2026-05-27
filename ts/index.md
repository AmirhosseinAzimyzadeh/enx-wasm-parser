# ts/index.ts

## Purpose
Public API for the `enx-wasm-parser` npm package. Exports the `ENXParser` class.

## Lifecycle
1. `new ENXParser()` — does not start the worker yet (lazy init).
2. `parser.parse(buffer, options)` — spawns the worker on first call, transfers
   the `ArrayBuffer` zero-copy, returns a `Promise<ParseResult>`.
3. `parser.dispose()` — terminates the worker and rejects pending promises.

## Worker management
- One persistent worker per `ENXParser` instance.
- Multiple `parse()` calls are safe; each is keyed by a monotonic `id` so
  concurrent calls don't interfere.
- If the worker crashes (uncaught error), all pending promises are rejected and
  the worker reference is cleared so the next `parse()` call re-spawns it.

## Zero-copy data flow
- **Input**: `ArrayBuffer` is transferred (not copied) to the worker via `postMessage`.
  The caller's reference becomes detached (neutered) after the call.
- **Output binary**: `Float32Array` is transferred back from the worker; the JS
  value in the resolved promise has full ownership.
