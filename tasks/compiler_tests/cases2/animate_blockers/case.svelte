<script>
	import { flip } from 'svelte/animate';

	let items = [{id: 1, name: 'a'}];
	let data = await fetch('/api');
	let params = $state(data.params);
</script>

{#each items as item (item.id)}
	<div animate:flip={params}>{item.name}</div>
{/each}
