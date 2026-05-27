<script>
	let a = $state(0);
	let arr = [];
	function update() {
		[a = 5] = arr;
	}
</script>
<button onclick={update}>{a}</button>
