<script>
	function handleKeydown(e) {
		console.log('keydown', e.key);
	}
</script>

<svelte:document on:keydown|capture|once={handleKeydown} />
