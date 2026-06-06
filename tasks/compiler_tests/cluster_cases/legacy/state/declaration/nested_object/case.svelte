<svelte:options runes={false} />
<script>
	let { p: { a }, q: { b } } = { p: { a: 1 }, q: { b: 2 } };
	function bump() { a = a; b = b; }
</script>
<button on:click={bump}>{a}{b}</button>
