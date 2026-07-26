<script>
	let x = $state(0);

	const list = [1, 2, 3];

	function delay(value) {
		return Promise.resolve(value);
	}
</script>

<button onclick={() => x++}>inc</button>

{list[await delay(x)]}
