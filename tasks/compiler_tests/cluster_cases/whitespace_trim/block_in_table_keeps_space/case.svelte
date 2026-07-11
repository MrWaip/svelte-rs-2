<script>
	let { rows } = $props();
</script>

<table><tbody>
	{#each rows as r}
		<tr><td>{r}</td></tr>
		<tr><td>{r}</td></tr>
	{/each}
</tbody></table>
