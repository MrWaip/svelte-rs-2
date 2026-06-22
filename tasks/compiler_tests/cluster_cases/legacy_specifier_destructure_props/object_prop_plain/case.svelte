<script>
	let { a, b } = { a: 1, b: 2 };
	export { a };
	function inc() { a++; }
</script>
<button on:click={inc}>{a}{b}</button>