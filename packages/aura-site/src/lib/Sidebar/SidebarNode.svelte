<script lang="ts">
	import { dev } from "$app/environment"
	import type { DocsPage, DocsTree } from "../../routes/docs/types"

	export let entry: DocsPage | DocsTree
	export let treeState: typeof entry["path"][]
	export let currentPath: string
</script>

{#if "pages" in entry}
	<details
		open={treeState?.includes?.(entry.path) ?? false}
		on:toggle={() => { 
			if (treeState.includes(entry.path)) treeState = treeState.filter(p => p === entry.path)
			else treeState.push(entry.path)
			treeState = treeState
		}}
	>
		<summary>
			{entry.title}
		</summary>
		<ul>
			{#each entry.pages as page}
				<li>
					<svelte:self {currentPath} entry={page} bind:treeState />
				</li>
			{/each}
		</ul>
	</details>
{:else if (entry.published === false && dev) || entry.published === true || entry.published === undefined}
	<a class="max-w-[22ch]" class:active={currentPath === entry.path} href="/docs{entry.path}">
		{entry.title}
		{#if entry.published === false}
			<div class="badge">unpublished</div>
		{/if}
	</a>
{/if}
