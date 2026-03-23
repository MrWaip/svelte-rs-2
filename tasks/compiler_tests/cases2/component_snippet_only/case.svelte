<script>
	let { data } = $props();
</script>

<Table items={data}>
	{#snippet row(item)}
		<span>{item}</span>
	{/snippet}
</Table>
