<svelte:options runes={true} />
<script>
	/** @type {Function | undefined} */
	let show = $state();

	/** Toggles the optional callback when clicked. */
	function toggle() {
		if (show) show();
	}
</script>

<button onclick={toggle}>toggle</button>
