<svelte:options runes={false} />
<script>
	let items = ['a', 'b'];
</script>

{#each items as item, idx}
	<input bind:value={item} />
{/each}
