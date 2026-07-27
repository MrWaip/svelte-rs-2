<script>
	function delay(value) {
		return Promise.resolve(value);
	}
	let loaded = await delay([1, 2]);
</script>

{#each loaded as item}
	<p>{item}</p>
{:else}
	<p>empty</p>
{/each}
