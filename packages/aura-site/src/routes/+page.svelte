<script lang="ts">
	import { MetaTags } from "svelte-meta-tags"
	import type { CodeExample } from "./+page.server"
	import { onMount } from "svelte"

	export let data
	$: ({ examples } = data)

	let activeExample: CodeExample = {
		title: "",
		source: "",
		html: "",
	}

	onMount(() => {
		activeExample = examples[0]
	})
</script>

<svelte:head>
	<MetaTags title="Aura Lang" description="A simple programming language" />
</svelte:head>

<main class="flex flex-row justify-evenly items-center max-lg:flex-col h-no-navbar bg-base-200">
	<div>
		<h1 class="text-5xl font-bold">Aura</h1>
		<p class="py-6">A simple programming language</p>
		<a href="/docs" class="btn btn-primary">Get Started</a>
	</div>

	<aside class="w-[48ch]">
		<menu class="tabs justify-center">
			{#each examples as example}
				<button
					class="tab tab-lifted"
					on:click={() => (activeExample = example)}
					class:tab-active={activeExample.title === example.title}
				>
					{example.title}
				</button>
			{/each}
		</menu>

		<div class="mockup-code">
			{@html activeExample.html}
		</div>
	</aside>
</main>
