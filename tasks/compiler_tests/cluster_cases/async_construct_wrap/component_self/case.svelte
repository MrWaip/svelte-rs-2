<script>
	let x = $state(0);
	let go = $state(false);

	function delay(value) {
		return Promise.resolve(value);
	}
</script>

<button onclick={() => x++}>inc</button>

{#if go}
	<svelte:self value={await delay(x)} />
{/if}
