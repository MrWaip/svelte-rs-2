<script>
	let count = $state(0);

	let doubled = $derived.by(() => {
		$inspect.trace();
		return count * 2;
	});

	$effect(() => {
		$inspect.trace("effect label");
		doubled;
	});
</script>
