/// <reference types="vitest" />
import { defineConfig } from "vitest/config";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import tailwindcss from "@tailwindcss/vite";
import { svelteTesting } from "@testing-library/svelte/vite";
// import svg from 'vite-plugin-svelte-svg';

import fs from "fs";
import { execSync } from "child_process";

const host = process.env.TAURI_DEV_HOST;
const appVersion = fs.readFileSync("./VERSION", "utf8").trim();
const getGitHash = () => {
  try {
    return execSync("git rev-parse --short HEAD").toString().trim();
  } catch (_e) {
    return "unknown";
  }
};

export default defineConfig({
  clearScreen: false,
  base: "./",
  plugins: [tailwindcss(), svelte(), svelteTesting()],
  define: {
    __APP_VERSION__: JSON.stringify(appVersion),
    __GIT_HASH__: JSON.stringify(getGitHash()),
  },
  // Env variables starting with the item of `envPrefix` will be exposed in tauri's source code through `import.meta.env`.
  envPrefix: ["VITE_", "TAURI_ENV_*"],
  build: {
    // Tauri uses Chromium on Windows and WebKit on macOS and Linux
    target:
      process.env.TAURI_ENV_PLATFORM == "windows" ? "chrome105" : "safari13",
    // don't minify for debug builds
    minify: !process.env.TAURI_ENV_DEBUG ? "esbuild" : false,
    // produce sourcemaps for debug builds
    sourcemap: !!process.env.TAURI_ENV_DEBUG,
  },
  server: {
    port: 3001, // Match the original sirv port
    strictPort: true,
    // if the host Tauri is expecting is set, use it
    host: host || false,
    hmr: host
      ? {
        protocol: "ws",
        host,
        port: 3001,
      }
      : undefined,

    watch: {
      // tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
  test: {
    environment: "jsdom",
    setupFiles: ["./src/test/setup.ts"],
    globals: true,
    coverage: {
      provider: "v8",
      reporter: ["text", "json", "html"],
      exclude: [
        "src/test/**",
        "src-tauri/**",
        "postcss.config.js",
        "tailwind.config.js",
        "svelte.config.js",
        "vite.config.ts",
        "eslint.config.js",
        ".svelte-kit/**",
      ],
    },
  },
});
