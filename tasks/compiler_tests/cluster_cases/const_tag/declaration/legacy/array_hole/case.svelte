<svelte:options runes={false} />
<script>
	let pairs = [1, 2, 3];
</script>
{#if pairs}
	{@const [a, , c] = pairs}
	<button>{a}{c}</button>
{/if}
