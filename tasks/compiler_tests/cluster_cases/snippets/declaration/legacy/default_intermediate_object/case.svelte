<svelte:options runes={false} />
<script>
	let v = {};
</script>
{#snippet s({ p: { a } = {} })}
	<button>{a}</button>
{/snippet}
{@render s(v)}
