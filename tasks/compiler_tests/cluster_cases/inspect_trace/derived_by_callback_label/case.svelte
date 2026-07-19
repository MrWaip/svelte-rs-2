<script>
	let count = $state(0);
	let doubled = $derived.by(() => {
		$inspect.trace();
		return count * 2;
	});
</script>

<button onclick={() => count++}>{doubled}</button>
