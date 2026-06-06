<svelte:options runes={false} />
<script>
	let v = { p: [1, 2], q: [3, 4] };
</script>
{#snippet s({ p: [a, b], q: [c, d] })}
	<button>{a}{b}{c}{d}</button>
{/snippet}
{@render s(v)}
