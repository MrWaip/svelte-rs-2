<script>
	let items = $state([{ outer: [{ inner: 1 }] }]);
</script>
{#each items as { outer: [{ inner }] }}
	<button>{inner}</button>
{/each}
