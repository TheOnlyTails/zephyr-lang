import { defineConfig } from "vite";

export default defineConfig({
  css: {
    lightningcss: {
      drafts: {
        customMedia: true,
        nesting: true,
      },
    },
  },
  build: {
    cssMinify: "lightningcss",
  },
});
