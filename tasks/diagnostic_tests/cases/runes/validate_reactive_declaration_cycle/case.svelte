<svelte:options runes={false} />
<script>
	let a = 1;
	let b = 2;
	$: a = b;
	$: b = a;
</script>

<p>{a}-{b}</p>
