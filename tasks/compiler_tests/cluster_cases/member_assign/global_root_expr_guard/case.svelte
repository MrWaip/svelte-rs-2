<script>
	function setup(id) {
		const entry = (globalThis.__cache[id] ??= {});
		return entry;
	}
</script>

<button onclick={() => setup(1)}>go</button>
