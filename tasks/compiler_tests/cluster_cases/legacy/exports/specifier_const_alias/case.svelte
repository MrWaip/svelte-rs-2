<svelte:options runes={false} />
<script>
	const total = 42;
	export { total as count };
</script>

<p>{total}</p>
