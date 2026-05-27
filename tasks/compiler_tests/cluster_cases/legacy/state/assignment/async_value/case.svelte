<script>
	let a = 0;
	let b = 0;
	async function update() {
		[a, b] = [1, await Promise.resolve(2)];
	}
</script>
<button on:click={update}>{a}{b}</button>
