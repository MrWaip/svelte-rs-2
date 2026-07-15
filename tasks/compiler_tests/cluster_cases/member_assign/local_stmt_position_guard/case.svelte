<script>
	let cache = {};
	function set(item) {
		cache[item.id] = item;
	}
</script>

<button onclick={() => set({})}>go</button>
