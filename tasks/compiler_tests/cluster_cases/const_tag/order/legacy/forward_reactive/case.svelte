<svelte:options runes={false} />
<script>
	let n = 1;
</script>
{#if n}
	{@const foo = bar}
	{@const bar = n}
	<button on:click={() => (n += 1)}>{foo}</button>
{/if}
