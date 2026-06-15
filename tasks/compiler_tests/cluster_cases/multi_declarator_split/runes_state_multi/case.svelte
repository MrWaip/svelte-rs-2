<script>
	let x = $state(1), y = $state(2);
	function inc() { x++; y++; }
</script>

<button onclick={inc}>{x}{y}</button>
