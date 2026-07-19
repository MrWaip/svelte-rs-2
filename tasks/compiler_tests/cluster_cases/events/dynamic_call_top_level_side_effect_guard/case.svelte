<script>
	let n = $state(0);
	function makeHandler() {
		return () => n++;
	}
</script>

<button onclick={makeHandler()}>go</button>
