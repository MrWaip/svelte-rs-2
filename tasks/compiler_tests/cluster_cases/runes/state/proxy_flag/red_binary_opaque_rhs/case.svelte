<script>
	let value = $state(0);
	function add() {
		value = value + 1;
	}
</script>

<button onclick={add}>{value}</button>
