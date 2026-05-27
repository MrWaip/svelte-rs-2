<svelte:options runes={false} />
<script>
	let pairs = {};
</script>
{#if pairs}
	{@const { p: { a } = {} } = pairs}
	<button>{a}</button>
{/if}
