import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import checker from "vite-plugin-checker";

export default defineConfig({
  envPrefix: ['VITE_', 'TAURI_PLATFORM', 'TAURI_ARCH', 'TAURI_FAMILY', 'TAURI_PLATFORM_VERSION', 'TAURI_PLATFORM_TYPE', 'TAURI_DEBUG'],
  plugins: [
    checker({
      eslint: {
        lintCommand: 'eslint "./src/**/*.{svelte,js,ts,jsx,tsx}"', // for example, lint .ts & .tsx
      },
    }),
    svelte({
      /* plugin options */
    }),
  ],
});
