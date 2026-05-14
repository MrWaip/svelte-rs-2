<svelte:options runes={false} />
<script>
	let a = null;
	let b = null;
	let c = null;

	function load(source) {
		[a, b, c] = source();
	}
</script>

<button onclick={() => load(() => [1, 2, 3])}>{a}</button>
