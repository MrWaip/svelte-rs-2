{#each items as item}
	{let a = $state(0)}
	{let b = $derived.by(() => a * item)}
	<button onclick={() => a++}>{b}</button>
{/each}
