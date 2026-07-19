<script>
	let count = $state(0);
	$effect(() => {
		$inspect.trace();
		count;
	});
</script>

<button onclick={() => count++}>{count}</button>
