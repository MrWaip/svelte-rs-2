<script>
	let { items = [] } = $props();
</script>

{#each items as item (item)}
	<p>{item.name}</p>
{/each}