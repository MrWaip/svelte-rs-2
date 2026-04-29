<svelte:options runes={false} />
<script>
	let count = 1;
	function fn() {
		$: value = count * 2;
	}
</script>

<button onclick={fn}>{count}</button>
