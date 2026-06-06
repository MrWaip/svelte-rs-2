<script>
	let items = $state([[[1, 2], [3, 4]]]);
</script>
{#each items as [[a, b], [c, d]]}
	<button>{a}{b}{c}{d}</button>
{/each}
