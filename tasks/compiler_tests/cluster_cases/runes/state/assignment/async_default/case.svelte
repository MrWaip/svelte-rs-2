<script>
	let a = $state(0);
	let arr = [];
	async function update() {
		[a = await Promise.resolve(3)] = arr;
	}
</script>
<button onclick={update}>{a}</button>
