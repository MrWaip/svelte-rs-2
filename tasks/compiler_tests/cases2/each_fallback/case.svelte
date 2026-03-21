<script>
	let { items = [] } = $props();
</script>

{#each items as item}
	<p>{item.name}</p>
{:else}
	<p>No items</p>
{/each}