<svelte:options runes={false} />
<script>
	let { p: { a } = {} } = {};
	function bump() { a = a; }
</script>
<button on:click={bump}>{a}</button>
