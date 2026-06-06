<script>
	let pairs = $state({ p: { a: 1 }, q: { b: 2 } });
</script>
{#if pairs}
	{@const { p: { a }, q: { b } } = pairs}
	<button>{a}{b}</button>
{/if}
