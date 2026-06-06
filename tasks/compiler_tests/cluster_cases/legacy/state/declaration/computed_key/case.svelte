<svelte:options runes={false} />
<script>
	const k = "z";
	let { [k]: v } = { z: 1 };
	function bump() { v = v; }
</script>
<button on:click={bump}>{v}</button>
