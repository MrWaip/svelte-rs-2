<svelte:options runes={false} />
<script>
	let v = { a: 1, b: 2 };
</script>
{#snippet s({ a: x, b: y })}
	<button>{x}{y}</button>
{/snippet}
{@render s(v)}
