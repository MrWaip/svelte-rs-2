<script>
	function handleKeydown(e) {
		console.log('keydown', e.key);
	}
</script>

<svelte:document onkeydown={handleKeydown} />
