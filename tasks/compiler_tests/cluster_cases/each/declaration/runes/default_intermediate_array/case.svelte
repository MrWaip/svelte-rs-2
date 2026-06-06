<script>
	let items = $state([[[1, 2], 3]]);
</script>
{#each items as [[a, b] = [8, 9], c]}
	<button>{a}{b}{c}</button>
{/each}
