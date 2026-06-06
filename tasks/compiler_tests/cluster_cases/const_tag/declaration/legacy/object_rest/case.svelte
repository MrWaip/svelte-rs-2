<svelte:options runes={false} />
<script>
	let pairs = { a: 1, b: 2, c: 3 };
</script>
{#if pairs}
	{@const { a, ...rest } = pairs}
	<button>{a}{rest.b}</button>
{/if}
