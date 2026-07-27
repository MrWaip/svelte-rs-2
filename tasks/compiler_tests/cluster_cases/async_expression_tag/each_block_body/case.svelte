<script>
	let items = $state([1, 2]);

	function delay(value) {
		return Promise.resolve(value);
	}
</script>

<button onclick={() => items.push(items.length)}>add</button>

{#each items as item}{await delay(item)}{/each}
