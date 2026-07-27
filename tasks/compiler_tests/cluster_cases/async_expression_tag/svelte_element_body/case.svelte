<script>
	let x = $state(0);

	function delay(value) {
		return Promise.resolve(value);
	}
</script>

<button onclick={() => x++}>inc</button>

<svelte:element this="p">{await delay(x)}</svelte:element>
