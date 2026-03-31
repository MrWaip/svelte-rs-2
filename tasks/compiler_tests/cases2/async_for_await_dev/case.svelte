<script>
	async function process(source) {
		const results = [];
		for await (const item of source) {
			results.push(item);
		}
		return results;
	}
</script>
