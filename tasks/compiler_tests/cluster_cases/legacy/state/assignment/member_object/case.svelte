<script>
	let obj = { x: 0 };
	let src = { v: 1 };
	function go() {
		({ v: obj.x } = src);
	}
</script>
<button on:click={go}>{obj.x}</button>