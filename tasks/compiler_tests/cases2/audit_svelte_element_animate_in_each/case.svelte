<script>
	import { flip } from "svelte/animate";
	let tag = $state("div");
	let items = $state([{ id: 1 }, { id: 2 }]);
</script>

{#each items as item (item.id)}
	<svelte:element this={tag} animate:flip>{item.id}</svelte:element>
{/each}
