<svelte:options runes={false} />
<script>
	let [[a, b] = [8, 9], c] = [[1, 2], 3];
	function bump() { a = a; b = b; c = c; }
</script>
<button on:click={bump}>{a}{b}{c}</button>
