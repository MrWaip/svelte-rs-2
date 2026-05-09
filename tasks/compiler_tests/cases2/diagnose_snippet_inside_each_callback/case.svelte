<script>
	let items = $state([1, 2, 3]);
</script>

{#each items as item}
	{#snippet row(v)}
		<p>{v}</p>
	{/snippet}
	{@render row(item)}
{/each}
