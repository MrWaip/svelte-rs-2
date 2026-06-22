<script>
	let { a, s } = { a: 1, s: 2 };
	export { a };
	function inc() { a++; s++; }
</script>
<button on:click={inc}>{a}{s}</button>