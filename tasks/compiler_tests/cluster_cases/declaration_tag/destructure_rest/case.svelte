{#each items as item}
	{const { a, ...rest } = item}
	<p>{a} {rest}</p>
{/each}
