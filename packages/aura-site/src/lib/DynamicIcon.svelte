<script lang="ts">
  export let icon: string
  
  $: iconPromise = import(/* @vite-ignore */ `~icons/material-symbols/${icon}`)
</script>

{#await iconPromise then Icon}
  <Icon />
{/await}