<script>
	let [ a, b ] = [ 1, 2 ];
	export { a, b };
	function inc() { a++; b++; }
</script>
<button on:click={inc}>{a}{b}</button>