import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import checker from "vite-plugin-checker";

export default defineConfig({
  plugins: [
    svelte({
      /* plugin options */
    }),
    checker({
      eslint: {
        lintCommand: 'eslint "./src/**/*.{svelte,js,ts,jsx,tsx}"', // for example, lint .ts & .tsx
      },
    }),
  ],
});
