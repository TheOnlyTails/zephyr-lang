<script lang="ts">
	import { page } from "$app/stores"
	import { Sidebar } from "$lib"
	import { externalLink } from "$lib/utils"
	import { MetaTags } from "svelte-meta-tags"
	import TableOfContents from "./TableOfContents.svelte"

	import type { SvelteComponent } from "svelte"
	import type { SvelteHTMLElements } from "svelte/elements"
	import ArrowUp from "~icons/material-symbols/arrow-upward"
	import DocsIcon from "~icons/material-symbols/developer-guide-outline-rounded"
	import TocIcon from "~icons/material-symbols/format-list-bulleted-rounded"
	import MenuClose from "~icons/material-symbols/left-panel-close-rounded"
	import MenuOpen from "~icons/material-symbols/left-panel-open-rounded"
	import Github from "~icons/mdi/github"

	export let data
	$: ({ tree, currentPage } = data)

	export const snapshot = {
		capture: () => ({ $currentSidebarTab: currentSidebarTab, $treeState: treeState	}),
		restore: ({ $currentSidebarTab, $treeState }) => {
			treeState = $treeState
			currentSidebarTab = $currentSidebarTab
		}
	}

	type SidebarTab = { 
		title: string,
		description: string,
		icon: typeof SvelteComponent<SvelteHTMLElements["svg"]>
	}

	let isSidebarOpen = false
	let treeState: typeof tree[number]["path"][] = []
	const sidebarTabs: Record<string, SidebarTab> = {
		docs: {
			title: "Docs",
			description: "Docs Menu",
			icon: DocsIcon
		},
		toc: {
			title: "Contents",
			description: "Table of Contents",
			icon: TocIcon
		}
	}
	let currentSidebarTab: keyof typeof sidebarTabs = "docs"

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
	<input id="docs-sidebar-toggle" type="checkbox" class="drawer-toggle" bind:checked={isSidebarOpen} />

	<aside class="drawer-side top-16 z-50 h-no-navbar">
		<label for="docs-sidebar-toggle" class="drawer-overlay" />

		<div class="flex flex-col items-stretch h-no-navbar bg-base-200 py-2">	
			<menu class="tabs tabs-boxed self-stretch px-4">
				{#each Object.entries(sidebarTabs) as [id, { title, icon: TabIcon, description }] (id)}
					<li>
						<button
							class="tab flex justify-center items-center gap-[1ch]"
							class:tab-active={currentSidebarTab === id}
							on:click={() => (currentSidebarTab = id)}
							title={description}
						>
							<TabIcon />
							{title}
						</button>
					</li>
				{/each}
			</menu>

			{#if currentSidebarTab === "docs"}
				<Sidebar {tree} {currentPage} bind:treeState />
			{:else if currentSidebarTab === "toc"}
				<TableOfContents {article} />
			{/if}

			<label for="docs-sidebar-toggle" class="self-end mt-auto m-4 lg:hidden drawer-button" aria-label="Close sidebar">
				<MenuClose />
			</label>
		</div>
	</aside>

	<main class="drawer-content max-w-[100vw] px-8 py-4">
		<nav class="flex flex-wrap items-center justify-between mb-4">
			<label for="docs-sidebar-toggle" class="lg:hidden drawer-button" aria-label="Open sidebar">
				<MenuOpen />
			</label>

			<ol class="list-none text-gray-500 flex gap-2">
				{#each currentPage.path.split("/").slice(1) as segment}
					<span>/</span>
					<li>{segment}</li>
				{:else}
					<span>/</span>
				{/each}
			</ol>
			
			<a
				href="https://github.com/TheOnlyTails/aura-lang/tree/main/packages/aura-site/src/routes{$page
					.route.id}/+page.md"
				class="btn btn-ghost"
				{...externalLink}
			>
				<Github />
				Edit on GitHub
			</a>
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
			<ArrowUp />
			Return to top
		</a>
	</div>
</section>
