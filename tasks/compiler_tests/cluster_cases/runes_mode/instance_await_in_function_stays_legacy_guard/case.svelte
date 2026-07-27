<script>
	let count = 0;

	async function load() {
		count = await Promise.resolve(1);
	}
</script>

<p>{count}</p>
<button onclick={load}>load</button>
