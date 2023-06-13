<script lang="ts">
	import { Sidebar } from "$lib"

	import "./syntax.css"
	import Menu from "~icons/material-symbols/menu-rounded"

	export let data
	$: ({ tree, currentPage } = data)
</script>

<section class="drawer min-h-no-navbar lg:drawer-open">
	<input id="docs-sidebar" type="checkbox" class="drawer-toggle" />

	<aside class="drawer-side lg:h-no-navbar">
		<label for="docs-sidebar" class="drawer-overlay" />
		<Sidebar {tree} {currentPage} />
	</aside>

	<main class="drawer-content px-8 py-4">
		<nav class="flex items-center justify-between mb-4">
			<label for="docs-sidebar" class="lg:hidden" aria-label="Open sidebar"><Menu /></label>

			<ol class="list-none text-gray-500 flex gap-2">
				{#each currentPage.path.split("/").slice(1) as segment}
					<span>/</span>
					<li>{segment}</li>
				{:else}
					<span>/</span>
				{/each}
			</ol>
		</nav>

		<article class="main-content prose prose-sm md:prose-base">
			<h1>
				{currentPage.title}
			</h1>

			<slot />
		</article>
	</main>
</section>
