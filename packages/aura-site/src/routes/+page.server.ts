import { getHighlighter } from "shiki"

export type CodeExample = {
	title: string
	source: string
	html: string
}

const highlighter = getHighlighter({
	theme: "dark-plus",
})

const codeExamples: Omit<CodeExample, "html">[] = [
	{
		title: "Hello world",
		source: `let main = () ->
  println("Hello, world!")`,
	},
	{
		title: "Functions",
		source: `let factorial = (n: int) ->
  if (n <= 1) 1 else n * self(n - 1)`,
	},
]

const highlight = async (source: string) => {
	const lines = Array.from(
		(await highlighter)
			.codeToHtml(source, { lang: "ts" })
			.matchAll(/<span class="line">(.+)<\/span>/gm)
	).flatMap((matches) => matches[1])

	return lines
		.map(
			(line, num) =>
				`<pre data-prefix="${num + 1}"><code>${line
					.replaceAll("{", "&lcub;")
					.replaceAll("}", "&rcub;")}</code></pre>`
		)
		.join("\n")
}

export const load = async () => ({
	examples: await Promise.all(
		codeExamples.map<Promise<CodeExample>>(async (example) => ({
			html: await highlight(example.source),
			...example,
		}))
	),
})
