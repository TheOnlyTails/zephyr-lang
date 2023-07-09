<script lang="ts">
	import type { DocsPage, DocsTree } from "../../routes/docs/types"
	import SidebarNode from "./SidebarNode.svelte"

	export let tree: (DocsPage | DocsTree)[]
	export let currentPage: DocsPage

	export let treeState: typeof tree[number]["path"][]
</script>

<nav class="menu gap-2">
	<ul>
		{#each tree as entry}
			<li>
				<SidebarNode currentPath={currentPage.path} {entry} bind:treeState />
			</li>
		{/each}
	</ul>
</nav>
