<script>
	let value = $state(0);
	const min = 2;
	function clamp() {
		value = min;
	}
</script>

<button onclick={clamp}>{value}</button>
