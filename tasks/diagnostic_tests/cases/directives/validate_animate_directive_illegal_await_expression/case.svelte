<script>
	import { flip } from 'svelte/animate';

	let items = [{ id: 1, value: 'x' }];

	async function load() {
		return { duration: 100 };
	}
</script>

{#each items as item (item.id)}
	<div animate:flip={await load()}>{item.value}</div>
{/each}
