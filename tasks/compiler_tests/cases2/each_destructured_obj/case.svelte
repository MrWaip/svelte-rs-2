<script>
	let { items = [] } = $props();
</script>

{#each items as { name, value }}
	<p>{name}: {value}</p>
{/each}