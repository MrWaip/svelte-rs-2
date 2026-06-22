<svelte:options runes={false} />
<script>
	let items = [{ done: false }];
	$: filtered = items;
</script>

{#each filtered as item}
	<input type="checkbox" bind:checked={item.done} />
{/each}
