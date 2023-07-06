import { writable } from "svelte/store";

export let treeState = writable(new Set<string>())