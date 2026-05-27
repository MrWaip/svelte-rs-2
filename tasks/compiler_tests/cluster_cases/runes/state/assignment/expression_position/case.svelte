<script>
	let a = $state(0);
	let arr = [1];
	function update() {
		let x = ([a] = arr);
		return x;
	}
</script>
<button onclick={update}>{a}</button>
