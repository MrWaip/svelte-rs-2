{#each items as item}
	{const { a: [b, c] } = item}
	<p>{b} {c}</p>
{/each}
