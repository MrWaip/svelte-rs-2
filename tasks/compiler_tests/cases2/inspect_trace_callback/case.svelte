<script>
	let count = $state(0);
	foo(() => {
		$inspect.trace();
		count++;
	});
</script>
