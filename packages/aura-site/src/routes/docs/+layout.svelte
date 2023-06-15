<script lang="ts">
	import { MetaTags } from "svelte-meta-tags"
	import { page } from "$app/stores"
	import { Sidebar } from "$lib"
	import { externalLink } from "$lib/utils"
	import TableOfContents from "./TableOfContents.svelte"

	import Menu from "~icons/material-symbols/menu-rounded"
	import Github from "~icons/mdi/github"
	import ArrowUp from "~icons/material-symbols/arrow-upward"

	export let data
	$: ({ tree, currentPage } = data)

	let currentSidebarTab: "docs" | "toc" = "docs"

	let article: HTMLElement
</script>

<svelte:head>
	<MetaTags 
		title="Aura Docs · {currentPage.title}" 
		openGraph={{
			type: "article",
			site_name: "Aura Lang",
			title: `Aura Docs · ${currentPage.title}`,
			url: $page.url.toString()
		}}
		twitter={{
			handle: "@theonlytails",
		}}
	/>
</svelte:head>

<section class="drawer min-h-no-navbar lg:drawer-open">
	<input id="docs-sidebar-toggle" type="checkbox" class="drawer-toggle" />

	<aside class="drawer-side top-16 z-50 h-no-navbar">
		<label for="docs-sidebar-toggle" class="drawer-overlay" />

		<div class="flex flex-col items-stretch h-no-navbar bg-base-200 py-2">
			<div class="tabs tabs-boxed self-center">
				<button
					class="tab"
					class:tab-active={currentSidebarTab === "docs"}
					on:click={() => (currentSidebarTab = "docs")}
				>
					Docs
				</button>
				<button
					class="tab"
					class:tab-active={currentSidebarTab === "toc"}
					on:click={() => (currentSidebarTab = "toc")}
				>
					Contents
				</button>
			</div>

			{#if currentSidebarTab === "docs"}
				<Sidebar {tree} {currentPage} />
			{:else if currentSidebarTab === "toc"}
				<TableOfContents {article} />
			{/if}

			<a
				href="https://github.com/TheOnlyTails/aura-lang/tree/main/packages/aura-site/src/routes{$page
					.route.id}/+page.md"
				class="btn btn-neutral mt-auto mb-1 mx-4"
				{...externalLink}
			>
				<Github />
				Edit on GitHub
			</a>
		</div>
	</aside>

	<main class="drawer-content max-w-[100vw] px-8 py-4">
		<nav class="flex items-center justify-between mb-4">
			<label for="docs-sidebar-toggle" class="lg:hidden drawer-button" aria-label="Open sidebar">
				<Menu />
			</label>

			<ol class="list-none text-gray-500 flex gap-2">
				{#each currentPage.path.split("/").slice(1) as segment}
					<span>/</span>
					<li>{segment}</li>
				{:else}
					<span>/</span>
				{/each}
			</ol>
		</nav>

		<article bind:this={article} class="main-content prose prose-sm md:prose-base">
			<h1>
				{currentPage.title}
			</h1>

			<slot />
		</article>
	</main>

	<!-- svelte-ignore a11y-invalid-attribute -->
	<div class="toast toast-end toast-bottom">
		<a href="#" class="btn btn-neutral shadow-lg animate-none">
			<ArrowUp /> Return to top
		</a>
	</div>
</section>
