<svelte:options runes={false} />
<script>
	let [{ a }, { b }] = [{ a: 1 }, { b: 2 }];
	function bump() { a = a; b = b; }
</script>
<button on:click={bump}>{a}{b}</button>
