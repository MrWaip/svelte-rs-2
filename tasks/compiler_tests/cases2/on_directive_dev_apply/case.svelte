<script>
	let count = $state(0);

	function getHandler() {
		return () => count++;
	}
</script>

<button on:click={getHandler()}>Click</button>
