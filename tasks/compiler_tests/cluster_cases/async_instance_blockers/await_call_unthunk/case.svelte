<script>
	let gate = $state(0);

	async function load() {
		return gate;
	}

	await load();
	let after = gate + 1;
</script>

<button onclick={() => gate++}>inc</button>
<p>{after}</p>
