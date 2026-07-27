<script>
	let x = $state(0);

	function delay(value) {
		return Promise.resolve(value);
	}

	function wrap(object) {
		return Object.keys(object).length;
	}
</script>

<button onclick={() => x++}>inc</button>

{wrap({ [await delay(x)]: 1 })}
