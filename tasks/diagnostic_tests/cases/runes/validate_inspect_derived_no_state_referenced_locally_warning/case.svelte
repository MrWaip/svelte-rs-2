<script>
	let count = $state(3);
	let doubled = $derived(count * 2);

	// eslint-disable-next-line svelte/no-inspect
	$inspect(doubled);
</script>
