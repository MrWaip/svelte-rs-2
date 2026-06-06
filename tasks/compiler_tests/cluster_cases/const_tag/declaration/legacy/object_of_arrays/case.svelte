<svelte:options runes={false} />
<script>
	let pairs = { p: [1, 2], q: [3, 4] };
</script>
{#if pairs}
	{@const { p: [a, b], q: [c, d] } = pairs}
	<button>{a}{b}{c}{d}</button>
{/if}
