<svelte:options runes={false} />
<script>
	var count = 0;

	function increment() {
		count += 1;
	}
</script>

<button onclick={increment}>clicks: {count}</button>
