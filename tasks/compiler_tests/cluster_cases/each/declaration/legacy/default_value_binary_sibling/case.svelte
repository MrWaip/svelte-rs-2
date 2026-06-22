<svelte:options runes={false} />
<script>
	let items = [{}];
</script>
{#each items as { w = 1, x = w * 2 }}
	<button>{w}{x}</button>
{/each}
