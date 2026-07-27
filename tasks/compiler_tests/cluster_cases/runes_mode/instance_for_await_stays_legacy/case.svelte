<script>
	let count = 0;

	async function* gen() {
		yield 1;
	}

	for await (const n of gen()) {
		count = n;
	}
</script>

<p>{count}</p>
<button onclick={() => count++}>inc</button>
