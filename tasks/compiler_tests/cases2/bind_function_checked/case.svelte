<script>
	let checked = $state(false);
</script>

<input type="checkbox" bind:checked={
	() => checked,
	(v) => checked = v
}>
