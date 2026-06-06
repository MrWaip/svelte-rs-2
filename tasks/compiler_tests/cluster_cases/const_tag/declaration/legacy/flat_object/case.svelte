<svelte:options runes={false} />
<script>
	let pairs = { a: 1, b: 2 };
</script>
{#if pairs}
	{@const { a, b } = pairs}
	<button>{a}{b}</button>
{/if}
