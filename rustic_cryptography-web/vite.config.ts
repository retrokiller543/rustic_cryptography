import { fileURLToPath, URL } from 'node:url'
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueDevTools from 'vite-plugin-vue-devtools'
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

// https://vitejs.dev/config/
export default defineConfig({
  server: {
    port: 6969
  },
  plugins: [
    vue(),
    vueDevTools(),
    wasm(),
    topLevelAwait()
  ],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url)),
      eventsource: "./node_modules/sockjs-client/lib/transport/browser/eventsource.js",
      events: "./node_modules/sockjs-client/lib/event/emitter.js",
      crypto: "./node_modules/sockjs-client/lib/utils/browser-crypto.js",
      'pkg': fileURLToPath(new URL('./pkg', import.meta.url)),  // Add this line to alias the pkg directory
    }
  },
  define: {
    global: {},
  },
})