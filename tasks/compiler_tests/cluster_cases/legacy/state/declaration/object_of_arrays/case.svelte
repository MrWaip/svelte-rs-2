<svelte:options runes={false} />
<script>
	let { p: [a, b], q: [c, d] } = { p: [1, 2], q: [3, 4] };
	function bump() { a = a; b = b; c = c; d = d; }
</script>
<button on:click={bump}>{a}{b}{c}{d}</button>
