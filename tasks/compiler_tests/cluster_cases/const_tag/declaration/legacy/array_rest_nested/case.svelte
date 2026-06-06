<svelte:options runes={false} />
<script>
	let pairs = [1, 2, 3];
</script>
{#if pairs}
	{@const [a, ...[b, c]] = pairs}
	<button>{a}{b}{c}</button>
{/if}
