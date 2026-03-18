<script>
	function handleKeydown(e) {
		console.log('keydown');
	}
	function handleKeyup(e) {
		console.log('keyup');
	}
</script>

<svelte:document on:keydown={handleKeydown} on:keyup={handleKeyup} />
