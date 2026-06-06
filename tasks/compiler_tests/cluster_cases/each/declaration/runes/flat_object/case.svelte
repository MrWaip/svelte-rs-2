<script>
	let items = $state([{ a: 1, b: 2 }]);
</script>
{#each items as { a, b }}
	<button>{a}{b}</button>
{/each}
