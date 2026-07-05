{#each items as item}
	{let current = item}
	<button onclick={() => current++}>{current}</button>
{/each}
