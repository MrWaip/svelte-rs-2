<script>
	const items = [fetch('/a'), fetch('/b')];
</script>

{#each items as item}
	{#await item then value}
		<p>{value}</p>
	{/await}
{/each}
