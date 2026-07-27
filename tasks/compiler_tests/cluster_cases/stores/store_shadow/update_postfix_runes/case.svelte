<script>
	let count = $state(0);
	$effect(() => {
		$count = 1;
	});
</script>

<button onclick={() => count++}>increment</button>
