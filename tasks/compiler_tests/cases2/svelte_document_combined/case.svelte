<script>
	let el = $state(null);

	function handleKeydown(e) {
		console.log('keydown', e.key);
	}
</script>

<svelte:document on:keydown={handleKeydown} bind:activeElement={el} />
