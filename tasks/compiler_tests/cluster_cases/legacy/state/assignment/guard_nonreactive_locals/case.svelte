<script>
	let count = 0;
	function update() {
		let x, y;
		[x, y] = [1, 2];
		return x + y;
	}
</script>
<button on:click={update}>{count}</button>
