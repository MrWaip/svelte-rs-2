<script>
	let count = $state(0);

	function read() {
		$inspect.trace();
		return count;
	}
</script>

<button onclick={() => count++}>{read()}</button>
