<script lang="ts">
	import Copy from "~icons/material-symbols/content-copy-outline-rounded"
	import Check from "~icons/material-symbols/check-circle-outline-rounded"

	export let content: string[]
	export let raw: string
	export let specialLines: ("info" | "warning" | "error" | "success" | undefined)[]

	let copied = false

	$: copied, setTimeout(() => (copied = false), 750)
</script>

<div class="mockup-code group not-prose relative">
	{#each content as line, i}
		<pre
			data-prefix={i + 1} 
			class="transition-colors"
			
			class:bg-info={specialLines[i] === "info"}
			class:!text-info-content={specialLines[i] === "info"}
			
			class:bg-warning={specialLines[i] === "warning"}
			class:!text-warning-content={specialLines[i] === "warning"}
			
			class:bg-error={specialLines[i] === "error"}
			class:!text-error-content={specialLines[i] === "error"}
			
			class:bg-success={specialLines[i] === "success"}
			class:!text-success-content={specialLines[i] === "success"}

			class:hover:bg-neutral-focus={specialLines[i] === null}
		><code>{@html line}</code></pre>
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
