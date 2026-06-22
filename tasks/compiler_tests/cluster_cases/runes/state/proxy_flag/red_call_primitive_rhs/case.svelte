<script>
	let value = $state(0);
	function clamp(x) {
		value = Math.min(100, +x);
	}
</script>

<button onclick={() => clamp(5)}>{value}</button>
