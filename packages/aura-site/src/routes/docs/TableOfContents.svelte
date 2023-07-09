<script lang="ts">
	import { page } from "$app/stores"

	export let article: HTMLElement

	let headings: HTMLHeadingElement[] = []
	let activeHeading: HTMLHeadingElement | undefined

	$: $page,
		(headings =
			article &&
			Array.from(article.querySelectorAll<HTMLHeadingElement>("h1, h2, h3, h4, h5, h6")).filter(
				(h) => h.closest(".main-content")
			))

	const findActiveHeading = () => {
		if (headings) activeHeading = [...headings].reverse().find((h) => h.offsetTop < window.scrollY)
	}
</script>

<svelte:window on:scroll={findActiveHeading} />

{#if article}
	{#key $page}
		<nav class="table-of-contents menu">
			<ul>
				{#each headings as { tagName, innerText, id }}
					{@const hLevel = Number(tagName[1])}
					<li>
						<a
							class:active={activeHeading?.id === id}
							class:font-bold={hLevel === 1}
							class:text-xl={hLevel === 1}
							class:font-semibold={hLevel === 2}
							class:text-lg={hLevel === 2}
							href="#{id}"
							style:--heading-level="'{"#".repeat(hLevel)}'"
							class="before:content-[--heading-level] before:text-base before:font-bold before:opacity-20 before:hover:opacity-60"
						>
							{innerText.replace(/^#+/, "")}
						</a>
					</li>
				{/each}
			</ul>
		</nav>
	{/key}
{/if}
