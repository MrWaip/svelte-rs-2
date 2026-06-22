<script>
	let { x: [ bar ] } = { x: [ 1 ] };
	export { bar };
	function inc() { bar++; }
</script>
<button on:click={inc}>{bar}</button>