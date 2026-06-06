<svelte:options runes={false} />
<script>
	let v = [1, 2, 3];
</script>
{#snippet s([a, ...rest])}
	<button>{a}{rest.length}</button>
{/snippet}
{@render s(v)}
