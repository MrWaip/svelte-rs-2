<script>
	let count = 0;
	function go() {
		let o = { a: 0 };
		[o.a] = [1];
		return o.a;
	}
</script>
<button on:click={go}>{count}</button>