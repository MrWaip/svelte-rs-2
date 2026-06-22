<script>
	let d = 1;
	export { d };
	function inc() { d++; }
</script>
<button on:click={inc}>{d}</button>