<script>
	let a = 0;
	let rest = 0;
	let obj = { a: 1, x: 2 };
	function update() {
		({ a, ...rest } = obj);
	}
</script>
<button on:click={update}>{a}{rest}</button>
