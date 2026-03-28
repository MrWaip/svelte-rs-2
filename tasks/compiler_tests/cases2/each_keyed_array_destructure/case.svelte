<script>
	let { items = [] } = $props();
</script>

{#each items as [id, name] (id)}
	<p>{name}</p>
{/each}