{#each items as item}
	{let a = $state(0), b = $derived(a * 2)}
	<button onclick={() => a++}>{a} {b}</button>
{/each}
