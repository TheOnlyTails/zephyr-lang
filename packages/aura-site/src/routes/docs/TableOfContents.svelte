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
			<li>
				<header class="menu-title text-center">Table of Contents</header>
			</li>
			<ul>
				{#each headings as { tagName, innerText, id }}
					<li>
						<a
							class="max-w-[22ch]"
							class:active={activeHeading?.id === id}
							href="#{id}"
							style:--heading-level={`"${"#".repeat(Number(tagName[1]))}"`}
						>
							{innerText.replace(/#+/, "")}
						</a>
					</li>
				{/each}
			</ul>
		</nav>
	{/key}
{/if}

<style lang="postcss">
	a::before {
		@apply text-base font-bold opacity-20;
		content: var(--heading-level);

		&:hover {
			@apply opacity-60;
		}
	}
</style>
