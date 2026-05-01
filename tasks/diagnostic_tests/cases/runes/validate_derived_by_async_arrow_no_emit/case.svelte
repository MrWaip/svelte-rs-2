<script>
	async function load() {
		return 1;
	}
	let value = $derived.by(async () => await load());
</script>

<p>{value}</p>
