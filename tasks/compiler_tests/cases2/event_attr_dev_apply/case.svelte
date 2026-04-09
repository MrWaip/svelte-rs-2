<script>
	let count = $state(0);

	function getHandler() {
		return () => count++;
	}
</script>

<button onclick={getHandler()}>Click</button>
