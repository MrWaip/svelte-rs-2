<script>
	let { items = [] } = $props();
</script>

{#each items as [id, name], idx (id)}
	<p>{idx}: {name}</p>
{/each}