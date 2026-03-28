<script>
	let { items = [] } = $props();
</script>

{#each items as [id, name] (id)}
	<p>{name}</p>
{/each}

{#each items as { id, name } (id)}
	<p>{name}</p>
{/each}

{#each items as [id, name], idx (id)}
	<p>{idx}: {name}</p>
{/each}

{#each items as [a, b, c], idx (b.key)}
{/each}

{#each items as { a, b, c }, idx (b.key)}
{/each}