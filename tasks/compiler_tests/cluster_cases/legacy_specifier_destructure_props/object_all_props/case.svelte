<script>
	let { a, b } = { a: 1, b: 2 };
	export { a, b };
	function inc() { a++; b++; }
</script>
<button on:click={inc}>{a}{b}</button>