<svelte:options runes={false} />
<script>
	export let entries;

	function put(item, value) {
		entries[item.id] = value;
	}
</script>

<button on:click={() => put({ id: 'a' }, 1)}>x</button>
