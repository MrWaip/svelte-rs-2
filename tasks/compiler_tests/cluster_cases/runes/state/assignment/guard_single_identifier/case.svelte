<script>
	let a = $state(0);
	function update() {
		a = 5;
	}
</script>
<button onclick={update}>{a}</button>
