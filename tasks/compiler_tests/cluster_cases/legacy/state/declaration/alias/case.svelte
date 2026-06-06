<svelte:options runes={false} />
<script>
	let { a: x, b: y } = { a: 1, b: 2 };
	function bump() { x = x; y = y; }
</script>
<button on:click={bump}>{x}{y}</button>
