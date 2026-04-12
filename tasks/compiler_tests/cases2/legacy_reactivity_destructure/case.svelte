<svelte:options runes={false} />
<script>
	let point = { left: 1, right: 2 };
	let { left, right } = point;

	function swap() {
		[left, right] = [right, left];
	}
</script>

<button onclick={swap}>{left}:{right}</button>
