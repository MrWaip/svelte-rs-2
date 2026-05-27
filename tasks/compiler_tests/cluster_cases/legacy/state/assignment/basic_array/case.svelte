<script>
	let a = 0;
	let b = 0;
	function update() {
		[a, b] = [1, 2];
	}
</script>
<button on:click={update}>{a}{b}</button>
