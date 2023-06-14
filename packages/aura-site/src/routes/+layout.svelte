<script lang="ts">
	import "../app.css"
	import Navbar from "./Navbar.svelte"
</script>

<div class="flex flex-col">
	<Navbar />

	<slot />
</div>
