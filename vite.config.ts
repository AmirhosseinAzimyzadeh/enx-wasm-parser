import { defineConfig } from 'vite';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';

export default defineConfig({
  plugins: [wasm(), topLevelAwait()],
  build: {
    lib: {
      entry: './ts/index.ts',
      name: 'ENXParser',
      formats: ['es'],
      fileName: 'index',
    },
    rollupOptions: {
      external: [],
    },
  },
  worker: {
    format: 'es',
    plugins: () => [wasm(), topLevelAwait()],
  },
});
