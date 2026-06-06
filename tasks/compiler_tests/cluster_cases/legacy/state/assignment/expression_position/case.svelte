<script>
	let a = 0;
	let arr = [1];
	function update() {
		let x = ([a] = arr);
		return x;
	}
</script>
<button on:click={update}>{a}</button>
