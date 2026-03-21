<script>
	let { items = [] } = $props();
</script>

{#each items as { name, value = 'N/A' }}
	<p>{name}: {value}</p>
{/each}