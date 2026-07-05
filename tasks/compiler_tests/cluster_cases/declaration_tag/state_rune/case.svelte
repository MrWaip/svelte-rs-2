{#each items as item}
	{let count = $state(0)}
	<button onclick={() => count++}>{count}</button>
{/each}
