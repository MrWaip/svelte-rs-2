<svelte:options runes={false} />
<script>
	let pairs = [1, 2];
</script>
{#if pairs}
	{@const [a, b] = pairs}
	<button>{a}{b}</button>
{/if}
