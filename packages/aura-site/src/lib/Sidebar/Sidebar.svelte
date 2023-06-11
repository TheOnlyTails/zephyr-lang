<script lang="ts">
	import { page } from "$app/stores"
	import { externalLink } from "$lib/utils"
	import type { DocsPage, DocsTree } from "../../routes/docs/types"
	import SidebarNode from "./SidebarNode.svelte"
	import Github from "~icons/mdi/github"

	export let tree: (DocsPage | DocsTree)[]
	export let currentPage: DocsPage

	let treeState: { [category: string]: boolean } = {}
</script>

<menu class="menu lg:h-no-navbar h-screen w-3/5 lg:w-80 bg-base-200">
	<li>
		<header class="menu-title">Docs Menu</header>
	</li>
	{#each tree as entry}
		<li>
			<SidebarNode {entry} bind:treeState />
			{#if currentPage.path === entry.path}
				<div></div>
			{/if}
		</li>
	{/each}

	<a
		href="https://github.com/TheOnlyTails/aura-lang/tree/main/packages/aura-site/src/routes{$page.route.id}/+page.md"
		class="btn btn-neutral mt-auto"
		{...externalLink}
	>
		<Github />
		Edit on GitHub
	</a>
</menu>
