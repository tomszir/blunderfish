import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import topLevelAwait from "vite-plugin-top-level-await";

export default defineConfig({
  plugins: [
    topLevelAwait({
      promiseExportName: "__tla",
      promiseImportName: (i) => `__tla_${i}`,
    }),
    svelte(),
  ],
  server: {
    watch: {
      usePolling: true,
    },
  },
  optimizeDeps: {
    exclude: ["@mono/wasm-core"],
  },
});
