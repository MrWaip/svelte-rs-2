{#each items as item}
	{let { a, b } = $derived(item)}
	<p>{a} {b}</p>
{/each}
