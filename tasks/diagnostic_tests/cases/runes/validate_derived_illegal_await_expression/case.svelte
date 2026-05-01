<script>
	async function load() {
		return 1;
	}
	let value = $derived(await load());
</script>

<p>{value}</p>
