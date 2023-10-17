import { DefaultTheme, defineConfig } from "vitepress"

// https://vitepress.vuejs.org/config/app-configs
export default defineConfig({
	appearance: "dark",
	lang: "en-US",
	title: "Zephyr Language",
	description: "A toy programming language, focused on simplicity and fun.",
	themeConfig: {
		nav: [
			{ text: "Guide", link: "/guide/what-is-zephyr" },
			{ text: "Reference", link: "/reference/index.html", target: "_self" }
		],

		sidebar: {
			"/guide/": {
				base: "/guide/",
				items: sidebarGuide(),
			}
		},
		editLink: {
			text: "Edit this page",
			pattern: "https://github.com/theonlytails/zephyr-lang/edit/main/docs/docs/:path"
		},
		lastUpdated: {},
		search: {
			provider: "local"
		}
	},
	sitemap: {
		hostname: "https://zephyr-lang.theonlytails.com"
	},
	cleanUrls: true,
	vite: {
		css: {
			lightningcss: {
				drafts: {
					customMedia: true,
					nesting: true
				}
			}
		},
		build: {
			cssMinify: "lightningcss"
		}
	}
})

function sidebarGuide(): DefaultTheme.SidebarItem[] {
	return [
		{
			text: "Introduction",
			collapsed: false,
			items: [
				{ text: "What is Zephyr?", link: "what-is-zephyr" },
				{ text: "Getting Started", link: "getting-started" },
			]
		}
	]
}
