<script>
	let pairs = $state([{ a: 1 }, { b: 2 }]);
</script>
{#if pairs}
	{@const [{ a }, { b }] = pairs}
	<button>{a}{b}</button>
{/if}
