<script>
	let gate = $state(0);

	async function load(v) {
		return v;
	}

	await load(await gate);
	let after = gate + 1;
</script>

<button onclick={() => gate++}>inc</button>
<p>{after}</p>
