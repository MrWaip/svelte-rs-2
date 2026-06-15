<svelte:options runes={false} />
<script>
	let items = [{ done: false }];
	export let filter = 'all';
	$: done = items.filter((i) => i.done);
	$: filtered = filter === 'all' ? items : done;
</script>

{#each filtered as item}
	<input type="checkbox" bind:checked={item.done} />
{/each}
