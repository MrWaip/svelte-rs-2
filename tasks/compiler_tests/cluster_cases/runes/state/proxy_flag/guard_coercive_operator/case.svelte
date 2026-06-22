<script>
	let value = $state(0);
	function inc() {
		value += 1;
	}
</script>

<button onclick={inc}>{value}</button>
