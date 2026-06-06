<svelte:options runes={false} />
<script>
	let pairs = [1];
</script>
{#if pairs}
	{@const [a = 10, b = 20] = pairs}
	<button>{a}{b}</button>
{/if}
