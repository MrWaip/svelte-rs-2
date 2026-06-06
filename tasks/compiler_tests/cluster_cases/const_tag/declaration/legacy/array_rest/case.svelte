<svelte:options runes={false} />
<script>
	let pairs = [1, 2, 3];
</script>
{#if pairs}
	{@const [a, ...rest] = pairs}
	<button>{a}{rest.length}</button>
{/if}
