<script>
	let pairs = $state([1, 2, 3]);
</script>
{#if pairs}
	{@const [a, ...rest] = pairs}
	<button>{a}{rest.length}</button>
{/if}
