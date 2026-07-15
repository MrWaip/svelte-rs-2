<svelte:options runes={false} />
<script>
	let a, b;
	const obj = { a: 1, b: 2 };
	function run() {
		({ a, b } = obj);
	}
</script>
<button on:click={run}>{a}{b}</button>
