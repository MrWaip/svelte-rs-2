<script>
	let count = 0;

	async function compute(v) {
		return v * 2;
	}
</script>

<p>{await compute(count)}</p>
