<script>
	import { flip } from 'svelte/animate';
	let tag = 'div';
	let items = $state([{ id: 1, name: 'a' }, { id: 2, name: 'b' }]);
</script>

{#each items as item (item.id)}
	<svelte:element this={tag} animate:flip>{item.name}</svelte:element>
{/each}
