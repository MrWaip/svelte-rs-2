<script>
	let count = 0;

	async function compute(v) {
		return v * 2;
	}
</script>

<p title={await compute(count)}>hi</p>
<button onclick={() => count++}>inc</button>
