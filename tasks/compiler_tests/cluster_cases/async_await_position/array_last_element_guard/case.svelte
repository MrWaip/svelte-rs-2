<script>
	let x = $state(0);

	function delay(value) {
		return Promise.resolve(value);
	}

	function wrap(values) {
		return values.length;
	}
</script>

<button onclick={() => x++}>inc</button>

{wrap([1, await delay(x)])}
