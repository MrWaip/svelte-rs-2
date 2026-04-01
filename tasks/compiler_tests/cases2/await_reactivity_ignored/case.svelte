<script>
	async function fetchData() {
		// svelte-ignore await_reactivity_loss
		const result = await fetch('/api');
		return result;
	}
</script>
