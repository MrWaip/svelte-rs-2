<svelte:options runes={false} />
<script>
	let v = { outer: [{ inner: 1 }] };
</script>
{#snippet s({ outer: [{ inner }] })}
	<button>{inner}</button>
{/snippet}
{@render s(v)}
