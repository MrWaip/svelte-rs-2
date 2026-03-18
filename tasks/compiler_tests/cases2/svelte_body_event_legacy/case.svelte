<script>
	function handleClick() {
		console.log('clicked');
	}
</script>

<svelte:body on:click={handleClick} />
