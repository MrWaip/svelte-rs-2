<script>
	let items = [1, 2];
</script>

{#each items as item}
	<p>ok</p>
	<svelte:self value={item} />
{/each}
