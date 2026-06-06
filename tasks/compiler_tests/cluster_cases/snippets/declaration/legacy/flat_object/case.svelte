<svelte:options runes={false} />
<script>
	let v = { a: 1, b: 2 };
</script>
{#snippet s({ a, b })}
	<button>{a}{b}</button>
{/snippet}
{@render s(v)}
