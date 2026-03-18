<script>
	let data = $state(null);
	async function fetchData() {
		$inspect.trace();
		data = await fetch('/api');
	}
</script>
