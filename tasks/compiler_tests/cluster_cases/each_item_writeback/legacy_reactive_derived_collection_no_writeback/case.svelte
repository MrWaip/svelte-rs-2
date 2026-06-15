<svelte:options runes={false} />
<script>
	let items = [{ done: false }];
	$: filtered = items;
</script>

{#each filtered as item}
	<p>{item.done}</p>
{/each}
