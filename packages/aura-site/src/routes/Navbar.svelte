<script context="module" lang="ts">
  export type NavbarEntry = {
    name: string;
    path: string;
    icon: any;
  };
</script>

<script lang="ts">
  import Book from "~icons/material-symbols/developer-guide";
  import Toy from "~icons/material-symbols/home";

  const entries = [
    {
      name: "Docs",
      path: "/docs",
      icon: Book,
    },
    {
      name: "Playground",
      path: "/playground",
      icon: Toy,
    },
  ] satisfies NavbarEntry[];
</script>

<nav class="navbar w-full h-14 bg-base-200 border-b border-b-divider">
  <a href="/" class="btn btn-ghost text-2xl normal-case">Aura</a>

  <ul class="menu menu-horizontal">
    {#each entries as { name, path, icon }}
      <li>
        <a href={path}>
          <svelte:component this={icon} />
          {name}
        </a>
      </li>
    {/each}
  </ul>
</nav>
