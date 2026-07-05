<script>
	let value = $state("a");
	function toggle() {
		value = value === "a" ? "b" : "c";
	}
</script>

<button onclick={toggle}>{value}</button>
