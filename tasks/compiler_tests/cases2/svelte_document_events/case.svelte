<script>
	function handleKeydown(e) {
		console.log('keydown', e.key);
	}
	function handleKeyup(e) {
		console.log('keyup');
	}
</script>

<svelte:document onkeydown={handleKeydown} on:keyup={handleKeyup} on:keydown|capture|once={handleKeydown} />
