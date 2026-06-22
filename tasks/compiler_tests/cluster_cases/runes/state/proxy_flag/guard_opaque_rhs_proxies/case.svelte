<script>
	let value = $state(0);
	function make() {
		return {};
	}
	function reset() {
		value = make();
	}
</script>

<button onclick={reset}>{value}</button>
