<script context="module" lang="ts">
	import type { SvelteComponent } from "svelte"
	import type { SvelteHTMLElements } from "svelte/elements"

	export type NavbarEntry = {
		name: string
		path: string
		icon: typeof SvelteComponent<SvelteHTMLElements["svg"]>
	}
</script>

<script lang="ts">
	import { externalLink } from "$lib/utils"

	import Book from "~icons/material-symbols/developer-guide"
	import Toy from "~icons/material-symbols/home"
	import Github from "~icons/mdi/github"

	const entries = [
		{
			name: "Docs",
			path: "/docs",
			icon: Book,
		},
		// {
		//   name: "Playground",
		//   path: "/playground",
		//   icon: Toy,
		// },
	] satisfies NavbarEntry[]
</script>

<nav class="navbar sticky top-0 z-50 w-full h-14 bg-base-200 border-b border-b-divider">
	<a href="/" class="btn hover:bg-transparent hover:border-transparent text-2xl normal-case">Aura</a
	>

	<ul class="flex-1 menu menu-horizontal">
		{#each entries as { name, path, icon }}
			<li>
				<a href={path}>
					<svelte:component this={icon} />
					{name}
				</a>
			</li>
		{/each}
	</ul>

	<a
		href="https://github.com/theonlytails/aura-lang"
		class="btn btn-ghost text-lg btn-circle flex-none"
		title="GitHub Repo"
		aria-label="GitHub Repo"
		{...externalLink}
	>
		<Github />
	</a>
</nav>
