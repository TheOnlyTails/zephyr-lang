import adapter from "@sveltejs/adapter-auto"
import { vitePreprocess } from "@sveltejs/kit/vite"
import { mdsvex, compile } from "mdsvex"
import linkHeadings from "rehype-autolink-headings"
import headingSlugs from "rehype-slug"
import remarkEmoji from "remark-emoji"
import remarkGfm from "remark-gfm"
import remarkGithub from "remark-github"
import remarkMermaid from "remark-mermaidjs"
import { getHighlighter } from "shiki"

const codeHighlighter = getHighlighter({
	theme: "dark-plus",
})

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
					ariaLabel: "Link to header",
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
			// layout: "./src/routes/docs/PageLayout.svelte",
			highlight: {
				highlighter: async (code, lang) => {
					const lines = Array.from(
						(await codeHighlighter)
							.codeToHtml(code, { lang })
							.matchAll(/<span class="line">(.+)<\/span>/gm)
					).flatMap((matches) =>
						matches[1]
							.replaceAll(/{/g, "&#123;")
							.replaceAll(/}/g, "&#125;")
							.replaceAll(/`/g, "&#96;")
							.replaceAll(/\\/g, "&#92;")
					)

					const rawCode = encodeURIComponent(code)

					// this is such a hack, but i cannot for the life of me figure out
					// another way to do this
					return `{#await import("$lib/CodeMockup.svelte") then { default: CodeMockup }}
	<CodeMockup content={${JSON.stringify(lines)}} raw={\`${rawCode}\`} />
{:catch error}
	<div class="mockup-code">
		<pre data-prefix={1} class="bg-error text-error-content">
			<code>Could not import the code mockup component: {error.message}</code>
		</pre>
	</div>
{/await}
`
				},
			},
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
