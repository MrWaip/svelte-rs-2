<script>
	let { items = [] } = $props();
</script>

{#each items as item, i (item.id + '_' + i)}
	<p>{item.name}</p>
{/each}