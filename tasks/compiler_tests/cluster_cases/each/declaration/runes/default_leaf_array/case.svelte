<script>
	let items = $state([[1]]);
</script>
{#each items as [a = 10, b = 20]}
	<button>{a}{b}</button>
{/each}
