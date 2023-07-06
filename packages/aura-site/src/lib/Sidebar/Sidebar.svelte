<script lang="ts">
	import type { DocsPage, DocsTree } from "../../routes/docs/types"
	import SidebarNode from "./SidebarNode.svelte"

	export let tree: (DocsPage | DocsTree)[]
	export let currentPage: DocsPage

	let treeState: { [category: string]: boolean } = {}
</script>

<nav class="menu gap-2">
	<li>
		<header class="menu-title text-center">Docs Menu</header>
	</li>
	{#each tree as entry}
		<li>
			<SidebarNode currentPath={currentPage.path} {entry} bind:treeState />
		</li>
	{/each}
</nav>
