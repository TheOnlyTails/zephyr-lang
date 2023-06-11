import type { SvelteHTMLElements } from "svelte/elements"

export const externalLink = {
	target: "_blank",
	rel: "noreferrer noopener",
} satisfies Partial<SvelteHTMLElements["a"]>
