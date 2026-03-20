<script>
	let count = $state(0);
	const obj = {
		handler() {
			$inspect.trace();
			count++;
		}
	};
</script>
