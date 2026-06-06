<script>
	let pairs = $state({});
</script>
{#if pairs}
	{@const { p: { a } = {} } = pairs}
	<button>{a}</button>
{/if}
