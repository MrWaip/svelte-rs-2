<script>
	let rows = [];
</script>

{#each rows as row, _ (row.id)}
	<p>{row.name}</p>
{/each}

{#each rows as row, i_dx}
	<span>{row.name}{i_dx}</span>
{/each}
