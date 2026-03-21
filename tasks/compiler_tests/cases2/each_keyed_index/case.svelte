<script>
	let { items = [] } = $props();
</script>

{#each items as item, i (item.id)}
	<p>{i}: {item.name}</p>
{/each}