<svelte:options runes={false} />

<script>
	let count = 0;
	const value = await Promise.resolve(1);
</script>

<p>{value} {count}</p>
