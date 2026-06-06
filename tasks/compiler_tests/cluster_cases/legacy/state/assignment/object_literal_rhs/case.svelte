<script>
	let a = 0;
	let b = 0;
	function update() {
		({ a, b } = { a: 1, b: 2 });
	}
</script>
<button on:click={update}>{a}{b}</button>
