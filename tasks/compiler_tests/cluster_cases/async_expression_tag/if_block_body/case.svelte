<script>
	let x = $state(0);

	function delay(value) {
		return Promise.resolve(value);
	}
</script>

<button onclick={() => x++}>inc</button>

{#if x >= 0}{await delay(x)}{/if}
