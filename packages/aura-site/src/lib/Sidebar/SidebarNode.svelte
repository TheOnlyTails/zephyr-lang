<script lang="ts">
	import { dev } from "$app/environment"
	import { DynamicIcon } from "$lib"
	import type { DocsPage, DocsTree } from "../../routes/docs/types"

	export let entry: DocsPage | DocsTree
	export let treeState: { [category: string]: boolean }
	export let currentPath: string
</script>

{#if "pages" in entry}
	<details
		open={treeState[entry.path]}
		on:toggle={() => (treeState[entry.path] = !treeState[entry.path])}
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
		{#if entry.icon}
			<DynamicIcon icon={entry.icon} />
		{/if}
		{entry.title}
		{#if entry.published === false}
			<div class="badge">unpublished</div>
		{/if}
	</a>
{/if}
