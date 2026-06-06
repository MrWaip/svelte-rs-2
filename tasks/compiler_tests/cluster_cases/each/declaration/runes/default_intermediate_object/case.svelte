<script>
	let items = $state([{}]);
</script>
{#each items as { p: { a } = {} }}
	<button>{a}</button>
{/each}
