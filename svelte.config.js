// Tauri doesn't have a Node.js server to do proper SSR, and neither does
// the self-hosted server build - the Rust backend serves the frontend as
// plain static files (see src-tauri/src/server/mod.rs), so both modes use
// adapter-static with a fallback to index.html to put the site in SPA mode.
// See: https://svelte.dev/docs/kit/single-page-apps
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
import adapterStatic from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

const isTauriMode = process.env.TAURI_ENV_PLATFORM !== undefined;

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),
  kit: {
    adapter: isTauriMode
      ? adapterStatic({ fallback: "index.html" })
      : adapterStatic({ pages: "frontend", assets: "frontend", fallback: "index.html" }),
  },
};

export default config;
