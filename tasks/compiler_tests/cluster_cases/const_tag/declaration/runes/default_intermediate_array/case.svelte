<script>
	let pairs = $state([[1, 2], 3]);
</script>
{#if pairs}
	{@const [[a, b] = [8, 9], c] = pairs}
	<button>{a}{b}{c}</button>
{/if}
