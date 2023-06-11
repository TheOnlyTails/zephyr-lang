import { sveltekit } from "@sveltejs/kit/vite"
import Icons from "unplugin-icons/vite"
import { defineConfig } from "vite"
import LightningCSS from "vite-plugin-lightningcss"

export default defineConfig({
	plugins: [
		sveltekit(),
		Icons({
			compiler: "svelte",
			defaultClass: "icon"
		}),
		LightningCSS({
			minify: true,
			browserslist: "last 2 versions, >= 0.25%, not dead",
			drafts: {
				customMedia: true,
				nesting: true
			},
			analyzeDependencies: true
		})
	]
})
