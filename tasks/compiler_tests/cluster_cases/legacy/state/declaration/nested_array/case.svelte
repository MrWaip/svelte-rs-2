<svelte:options runes={false} />
<script>
	let [[a, b], [c, d]] = [[1, 2], [3, 4]];
	function bump() { a = a; b = b; c = c; d = d; }
</script>
<button on:click={bump}>{a}{b}{c}{d}</button>
