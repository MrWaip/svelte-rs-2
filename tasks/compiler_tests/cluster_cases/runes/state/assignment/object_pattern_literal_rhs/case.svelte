<script>
	let a = $state(0);
	let b = $state(0);
	function update() {
		({ a, b } = { a: 1, b: 2 });
	}
</script>
<button onclick={update}>{a}{b}</button>
