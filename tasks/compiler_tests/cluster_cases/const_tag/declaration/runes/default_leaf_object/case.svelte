<script>
	let pairs = $state({});
</script>
{#if pairs}
	{@const { a = 10, b = 20 } = pairs}
	<button>{a}{b}</button>
{/if}
