<script>
	let { message } = $props();
	function report() {
		console.error("Error: " + message);
	}
</script>

<button onclick={report}>go</button>
