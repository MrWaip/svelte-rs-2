<script>
	async function getItems() {
		return [1, 2, 3];
	}
</script>

{#each await getItems() as item}
	<p>{item}</p>
{/each}
