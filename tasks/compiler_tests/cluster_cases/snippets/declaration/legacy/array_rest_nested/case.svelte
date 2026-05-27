<svelte:options runes={false} />
<script>
	let v = [1, 2, 3];
</script>
{#snippet s([a, ...[b, c]])}
	<button>{a}{b}{c}</button>
{/snippet}
{@render s(v)}
