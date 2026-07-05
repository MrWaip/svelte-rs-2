{#each items as item}
	{const value = await item.load()}
	<p>{value}</p>
{/each}
