<script>
	let x = 1, y = 2;
	function inc() { x++; y++; }
</script>

<button on:click={inc}>{x}{y}</button>
