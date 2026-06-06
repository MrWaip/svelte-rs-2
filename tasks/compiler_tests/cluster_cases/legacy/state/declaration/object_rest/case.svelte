<svelte:options runes={false} />
<script>
	let { a, ...rest } = { a: 1, b: 2, c: 3 };
	function bump() { a = a; }
</script>
<button on:click={bump}>{a}{rest.b}</button>
