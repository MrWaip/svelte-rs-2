<svelte:options runes={false} />
<script>
	let [a, , c] = [1, 2, 3];
	function bump() { a = a; c = c; }
</script>
<button on:click={bump}>{a}{c}</button>
