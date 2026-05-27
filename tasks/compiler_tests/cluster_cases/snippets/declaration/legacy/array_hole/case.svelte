<svelte:options runes={false} />
<script>
	let v = [1, 2, 3];
</script>
{#snippet s([a, , c])}
	<button>{a}{c}</button>
{/snippet}
{@render s(v)}
