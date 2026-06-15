<svelte:options runes={false} />
<script>
	let items = [{ value: 'x' }];
	function keep(it) {
		return true;
	}
</script>

{#each items.filter(keep) as item}
	<input bind:value={item.value} />
{/each}
