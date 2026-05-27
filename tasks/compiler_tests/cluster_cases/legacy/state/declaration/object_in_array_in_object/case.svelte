<svelte:options runes={false} />
<script>
	let { outer: [{ inner }] } = { outer: [{ inner: 1 }] };
	function bump() { inner = inner; }
</script>
<button on:click={bump}>{inner}</button>
