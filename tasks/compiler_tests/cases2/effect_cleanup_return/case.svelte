<script>
	let count = $state(0);

	$effect(() => {
		const interval = setInterval(() => {
			count += 1;
		}, 1000);

		return () => {
			clearInterval(interval);
		};
	});
</script>

<p>{count}</p>
