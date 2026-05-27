<svelte:options runes={false} />
<script>
	let [a, ...rest] = [1, 2, 3];
	function bump() { a = a; }
</script>
<button on:click={bump}>{a}{rest.length}</button>
