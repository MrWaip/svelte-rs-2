{#each items as item}
	{let a, b}
	<button onclick={() => { a = item; b = item; }}>{a} {b}</button>
{/each}
