<script>
	let items = [1, 2];
</script>

{#each items as item}
	<svelte:self value={item} />
{/each}
