<script>
	let result = {};
	let data = {};
	function fill(keys) {
		keys.forEach((key) => result[key] = data[key] ? true : false);
	}
</script>

<button onclick={() => fill([])}>go</button>
