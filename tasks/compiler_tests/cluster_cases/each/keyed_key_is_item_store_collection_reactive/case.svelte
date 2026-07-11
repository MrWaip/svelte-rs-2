<script>
	let { store, columns } = $props();
	const s = $derived(store);
</script>
{#each $s.keys as key (key)}
	{@const column = columns[key]}
	<div>{column}</div>
{/each}
