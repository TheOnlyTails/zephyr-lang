import { defineConfig } from "vitepress"

// https://vitepress.vuejs.org/config/app-configs
export default defineConfig({
	appearance: "dark",
	lang: "en-US",
	title: "Zephyr Language",
	description: "A toy programming language, focused on simplicity and fun.",
	themeConfig: {
		nav: [
			{ text: "Guide", link: "/guide/" },
			{ text: "Reference", link: "/reference/" }
		],
		sidebar: {
			"/guide/": [{
				text: "Guide",
				items: [
					{ text: "Getting Started", link: "/guide/" }
				]
			}]
		},
		editLink: {
			text: "Edit this page",
			pattern: "https://github.com/theonlytails/zephyr-lang/edit/main/docs/:path"
		},
		lastUpdated: {},
		search: {
			provider: "local"
		}
	},
	cleanUrls: true
})
