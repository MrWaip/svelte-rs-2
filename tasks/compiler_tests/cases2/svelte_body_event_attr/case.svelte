<script>
	function handleClick() {
		console.log('clicked');
	}
</script>

<svelte:body onclick={handleClick} />
