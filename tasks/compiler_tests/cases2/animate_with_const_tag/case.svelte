<script>
	import { flip } from 'svelte/animate';
	let items = $state([{ id: 1, name: 'a' }, { id: 2, name: 'b' }]);
</script>

{#each items as item (item.id)}
	{@const label = item.name.toUpperCase()}
	<div animate:flip>{label}</div>
{/each}
