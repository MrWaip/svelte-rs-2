<script>
	import { writable } from "svelte/store";
	let tag = $state("div");
	const store = writable();
</script>

<svelte:element this={tag} bind:this={$store}>x</svelte:element>
