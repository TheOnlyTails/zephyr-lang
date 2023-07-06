<script lang="ts">
	import Copy from "~icons/material-symbols/content-copy-outline-rounded"
	import Check from "~icons/material-symbols/check-circle-outline-rounded"
	export let content: string[]
	export let raw: string

	let copied = false

	$: copied, setTimeout(() => (copied = false), 750)
</script>

<div class="mockup-code group not-prose relative">
	{#each content as line, i}
		<pre data-prefix={i + 1}><code>{@html line}</code></pre>
	{/each}
	<button
		class="btn btn-sm btn-outline btn-circle hidden group-hover:inline-flex absolute top-4 right-4"
		on:click={() => {
			navigator.clipboard.writeText(decodeURIComponent(raw))
			copied = true
		}}
		aria-label={!copied ? "Copy to Clipboard" : "Copied!"}
	>
		{#if !copied}
			<Copy />
		{:else}
			<Check />
		{/if}
	</button>
</div>
