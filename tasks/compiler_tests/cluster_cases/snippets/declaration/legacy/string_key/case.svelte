<svelte:options runes={false} />
<script>
	let v = { "a-b": 1, "c d": 2 };
</script>
{#snippet s({ "a-b": ab, "c d": cd })}
	<button>{ab}{cd}</button>
{/snippet}
{@render s(v)}
