import { defineConfig } from 'vite';
import wasm from "vite-plugin-wasm";
// import wasmPack from 'vite-plugin-wasm-pack';

export default defineConfig({
  build: {
    target: 'esnext',
  },
  plugins: [
    wasm(),
  ]
});
