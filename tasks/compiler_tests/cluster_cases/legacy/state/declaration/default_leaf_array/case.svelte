<svelte:options runes={false} />
<script>
	let [a = 10, b = 20] = [1];
	function bump() { a = a; b = b; }
</script>
<button on:click={bump}>{a}{b}</button>
