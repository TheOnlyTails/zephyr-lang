import adapter from "@sveltejs/adapter-auto"
import { vitePreprocess } from "@sveltejs/kit/vite"
import { mdsvex } from "mdsvex"
import { svelteShiki } from "svelte-shiki"
import headingSlugs from "rehype-slug"
import linkHeadings from "rehype-autolink-headings"
import remarkGfm from "remark-gfm"
import remarkGithub from "remark-github"
import remarkMermaid from "remark-mermaidjs"
import remarkEmoji from "remark-emoji"

const rehypePlugins = [
	headingSlugs,
	[
		linkHeadings,
		{
			behavior: "prepend",
			/**
			 * @type {import("rehype-autolink-headings").Options["content"]}
			 */
			content: (element) => ({
				type: "element",
				tagName: "span",
				properties: {
					className: [
						"mr-1 opacity-20 hover:opacity-60 text-base font-bold inline-block align-middle relative -mt-1",
					],
				},
				children: [
					{
						type: "text",
						value: "#".repeat(Number(element.tagName.at(1) ?? 1)),
					},
				],
			}),
		},
	],
]

/** @type {import('@sveltejs/kit').Config} */
const config = {
	extensions: [".svelte", ".md"],

	// Consult https://kit.svelte.dev/docs/integrations#preprocessors
	// for more information about preprocessors
	preprocess: [
		svelteShiki(),
		mdsvex({
			extension: ".md",
			rehypePlugins,
			remarkPlugins: [
				[
					remarkEmoji,
					{
						accessible: true,
					},
				],
				remarkGfm,
				remarkGithub,
				remarkMermaid,
			],
		}),
		vitePreprocess(),
	],

	kit: {
		// adapter-auto only supports some environments, see https://kit.svelte.dev/docs/adapter-auto for a list.
		// If your environment is not supported or you settled on a specific environment, switch out the adapter.
		// See https://kit.svelte.dev/docs/adapters for more information about adapters.
		adapter: adapter(),
	},
	vitePlugin: {
		inspector: true,
	},
}

export default config
