<script>
	let x = $state(0);

	function delay(value) {
		return Promise.resolve(value);
	}
</script>

<button onclick={() => x++}>inc</button>

<svelte:element this={await delay(x) + "div"}>content</svelte:element>
