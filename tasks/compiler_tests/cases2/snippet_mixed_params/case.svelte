<script>
	let items = $state([{ id: 1 }]);
</script>

{#snippet row(label, { id }, [value])}
	<p>{label}: {id} = {value}</p>
{/snippet}

{@render row("test", items[0], [42])}
