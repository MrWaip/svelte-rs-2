<svelte:options runes={false} />
<script>
	export let items;
	const makeEmpty = () => ({ a: '' });
	$: if (items) {
		items = items.map((x) => ({ ...x, info: x.info ?? makeEmpty() }));
	}
</script>
{#each items as item}
	<input bind:value={item.a} />
{/each}
