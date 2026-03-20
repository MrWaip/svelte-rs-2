<script>
	let value = $state('');
</script>

<input bind:value={
	() => value,
	(v) => value = v
}>
