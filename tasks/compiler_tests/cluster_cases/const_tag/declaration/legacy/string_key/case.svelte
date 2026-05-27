<svelte:options runes={false} />
<script>
	let pairs = { "a-b": 1, "c d": 2 };
</script>
{#if pairs}
	{@const { "a-b": ab, "c d": cd } = pairs}
	<button>{ab}{cd}</button>
{/if}
