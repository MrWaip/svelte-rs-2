<script>
	let count = $state(0);
	$effect(() => {
		$count = 1;
	});
</script>

<button onclick={() => count = 5}>set</button>
