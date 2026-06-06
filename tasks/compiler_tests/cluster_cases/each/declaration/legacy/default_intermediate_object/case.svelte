<svelte:options runes={false} />
<script>
	let items = [{}];
</script>
{#each items as { p: { a } = {} }}
	<button>{a}</button>
{/each}
