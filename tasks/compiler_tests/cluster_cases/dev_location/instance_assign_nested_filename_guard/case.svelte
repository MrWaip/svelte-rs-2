<script>
	let cache = {};
	function fill(items) {
		items.forEach((item) => cache[item.id] = item);
	}
</script>

<button onclick={() => fill([])}>go</button>
