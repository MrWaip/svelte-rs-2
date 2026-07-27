<script>
	let count = 0;

	async function compute(v) {
		return v * 2;
	}
</script>

<p>{count}</p>
<button onclick={() => count++}>inc</button>
<button onclick={async () => { count = await compute(count); }}>double</button>
