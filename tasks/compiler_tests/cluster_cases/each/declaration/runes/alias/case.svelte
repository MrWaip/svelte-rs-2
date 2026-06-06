<script>
	let items = $state([{ a: 1, b: 2 }]);
</script>
{#each items as { a: x, b: y }}
	<button>{x}{y}</button>
{/each}
