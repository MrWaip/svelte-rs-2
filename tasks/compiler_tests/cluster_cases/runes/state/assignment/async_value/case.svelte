<script>
	let a = $state(0);
	let b = $state(0);
	async function update() {
		[a, b] = [1, await Promise.resolve(2)];
	}
</script>
<button onclick={update}>{a}{b}</button>
