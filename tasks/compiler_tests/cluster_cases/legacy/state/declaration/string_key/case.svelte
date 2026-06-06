<svelte:options runes={false} />
<script>
	let { "a-b": ab, "c d": cd } = { "a-b": 1, "c d": 2 };
	function bump() { ab = ab; cd = cd; }
</script>
<button on:click={bump}>{ab}{cd}</button>
