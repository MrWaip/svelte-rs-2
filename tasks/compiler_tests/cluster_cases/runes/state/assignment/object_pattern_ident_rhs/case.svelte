<script>
	let a = $state(0);
	let b = $state(0);
	let obj = { a: 1, b: 2 };
	function update() {
		({ a, b } = obj);
	}
</script>
<button onclick={update}>{a}{b}</button>
