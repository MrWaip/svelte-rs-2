<svelte:options runes={false} />

<script>
	let x = 1;
	$: y = x * 2;
	y;
</script>
