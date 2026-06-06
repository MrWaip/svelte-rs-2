<svelte:options runes={false} />
<script>
	let pairs = { p: { a: 1 }, q: { b: 2 } };
</script>
{#if pairs}
	{@const { p: { a }, q: { b } } = pairs}
	<button>{a}{b}</button>
{/if}
