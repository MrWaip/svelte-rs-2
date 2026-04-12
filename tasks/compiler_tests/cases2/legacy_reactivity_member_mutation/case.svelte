<svelte:options runes={false} />
<script>
	let object = { x: 0 };

	function bump() {
		object.x += 1;
	}
</script>

<button onclick={bump}>value: {object.x}</button>
