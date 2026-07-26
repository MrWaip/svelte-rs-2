<script>
	let x = $state(0);

	function delay(value) {
		return Promise.resolve(value);
	}
</script>

<button onclick={() => x++}>inc</button>

<div {...delay(x)} {...await delay({ title: x })}></div>
