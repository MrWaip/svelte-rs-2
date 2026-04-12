<svelte:options runes={false} />
<script>
	let count = 1;
	var step = 2;

	function bump() {
		count += 1;
		step += 1;
	}

	$: doubled = count * 2;
	$: total = doubled + step;
</script>

<button onclick={bump}>{doubled}-{total}</button>
