<script>
	const initial = 0;
	let value = $state(initial);
	function bump() {
		value += 1;
	}
</script>

<button onclick={bump}>{value}</button>
