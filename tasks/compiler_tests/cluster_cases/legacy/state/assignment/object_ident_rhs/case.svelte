<script>
	let a = 0;
	let b = 0;
	let obj = { a: 1, b: 2 };
	function update() {
		({ a, b } = obj);
	}
</script>
<button on:click={update}>{a}{b}</button>
