<script>
	let { a, ...rest } = { a: 1, b: 2, c: 3 };
	export { a };
	function inc() { a++; }
</script>
<button on:click={inc}>{a}{JSON.stringify(rest)}</button>