<script>
	let a = $state(0);
	let rest = $state(0);
	let obj = { a: 1, x: 2 };
	function update() {
		({ a, ...rest } = obj);
	}
</script>
<button onclick={update}>{a}{rest}</button>
