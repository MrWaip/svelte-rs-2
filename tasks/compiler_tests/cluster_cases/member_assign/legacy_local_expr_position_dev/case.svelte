<svelte:options runes={false} />

<script>
	let cache = {};
	function fill(items) {
		items.forEach(item => cache[item.id] = item);
	}
</script>

<button on:click={() => fill([])}>go</button>
