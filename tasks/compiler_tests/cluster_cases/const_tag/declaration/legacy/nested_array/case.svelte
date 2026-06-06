<svelte:options runes={false} />
<script>
	let pairs = [[1, 2], [3, 4]];
</script>
{#if pairs}
	{@const [[a, b], [c, d]] = pairs}
	<button>{a}{b}{c}{d}</button>
{/if}
