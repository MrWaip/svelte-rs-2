<script>
	let a = 0;
	let b = 0;
	let arr = [1, 2, 3];
	function update() {
		[a, ...b] = arr;
	}
</script>
<button on:click={update}>{a}{b}</button>
