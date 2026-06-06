<svelte:options runes={false} />
<script>
	let items = [{}];
</script>
{#each items as { a = 10, b = 20 }}
	<button>{a}{b}</button>
{/each}
