import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import tailwindcss from '@tailwindcss/vite';
// import svg from 'vite-plugin-svelte-svg';

const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  clearScreen: false,
  base: "./",
  plugins: [tailwindcss(), svelte(),
  // svg({
  //   svgoConfig: {}, // optional SVGO config
  //   requireSuffix: true, // generates Icon and IconUrl exports
  // }),
  ],
  // Env variables starting with the item of `envPrefix` will be exposed in tauri's source code through `import.meta.env`.
  envPrefix: ['VITE_', 'TAURI_ENV_*'],
  build: {
    // Tauri uses Chromium on Windows and WebKit on macOS and Linux
    target:
      process.env.TAURI_ENV_PLATFORM == 'windows'
        ? 'chrome105'
        : 'safari13',
    // don't minify for debug builds
    minify: !process.env.TAURI_ENV_DEBUG ? 'esbuild' : false,
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
        protocol: 'ws',
        host,
        port: 3001,
      }
      : undefined,

    watch: {
      // tell vite to ignore watching `src-tauri`
      ignored: ['**/src-tauri/**'],
    },
  },
});
