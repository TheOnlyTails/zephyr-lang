import { defineConfig } from "vitepress";

import pkg from "../../package.json" with { type: "json" };

// https://vitepress.vuejs.org/config/app-configs
export default defineConfig({
	title: "Zephyr Lang",
	description: "A toy programming language focused on simplicity",
	themeConfig: {
		editLink: {
			pattern: "https://github.com/theonlytails/zephyr-lang/edit/main/docs/:path",
			text: "Edit this Page on GitHub"
		},
		socialLinks: [
			{ icon: "github", link: "https://github.com/theonlytails/zephyr-lang" }
		],
		nav: [
			{
				text: "v" + pkg.version,
				items: [
					{
						text: "Changelog",
						link: "https://github.com/theonlytails/zephyr-lang/blob/main/CHANGELOG.md"
					},
					{
						text: "Contributing",
						link: "https://github.com/theonlytails/zephyr-lang/blob/main/CONTRIBUTING.md"
					}
				]
			}
		],
		sidebar: [
			{
				text: "Guide",
				items: [
					{ text: "Getting Started", link: "/guide/" },
					{ text: "Literals", link: "/guide/literals" }
				]
			}
		],
		search: {
			provider: "local"
		}
	},
	markdown: {
		lineNumbers: true,
		theme: "dracula"
	},
	head: [["meta", { name: "theme-color", content: "#af3dcb" }]],
	lastUpdated: true,
	cleanUrls: true,
	sitemap: {
		hostname: "https://zephyr-lang.vercel.app"
	}
});
