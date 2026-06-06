<script>
	let items = $state([[1, 2]]);
</script>
{#each items as [a, b]}
	<button>{a}{b}</button>
{/each}
