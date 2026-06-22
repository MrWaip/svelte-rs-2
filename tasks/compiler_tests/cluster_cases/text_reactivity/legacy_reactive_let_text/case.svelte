<script>
	let count = 0;
	function inc() { count++; }
</script>

<button on:click={inc}>{count}</button>
