// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
// See: https://beta.tauri.app/start/frontend/sveltekit/ for more info
import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";
import preprocess from "svelte-preprocess";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: [
    preprocess({
      postcss: true,
    }),
    vitePreprocess(),
  ],
  kit: {
    adapter: adapter({
      fallback: 'app.html',
      strict: false,
    }),
  },
  prerender: {
    crawl: true,
    entries: ['*', '/0/highlights/', '/0/summary/']
  }
};

export default config;
