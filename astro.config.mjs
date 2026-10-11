import { defineConfig } from "astro/config";

export default defineConfig({
  site: "https://harumaki1220.github.io",
  base: process.env.BASE_PATH ?? "/portfolio",
});
