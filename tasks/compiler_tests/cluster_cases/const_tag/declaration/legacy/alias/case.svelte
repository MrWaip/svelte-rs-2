<svelte:options runes={false} />
<script>
	let pairs = { a: 1, b: 2 };
</script>
{#if pairs}
	{@const { a: x, b: y } = pairs}
	<button>{x}{y}</button>
{/if}
