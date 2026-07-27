<script>
	let count = $state(1);
	async function getDouble(value) {
		return value * 2;
	}
	const double = $derived(await getDouble(count));
</script>

<button onclick={() => count++}>inc</button>
<p>Count: {count} Double: {double}</p>
