<script>
	let pairs = $state({ outer: [{ inner: 1 }] });
</script>
{#if pairs}
	{@const { outer: [{ inner }] } = pairs}
	<button>{inner}</button>
{/if}
