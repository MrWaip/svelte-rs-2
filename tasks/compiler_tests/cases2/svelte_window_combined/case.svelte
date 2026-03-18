<script>
	let scrollY = $state(0);

	function handleResize() {
		console.log('resized');
	}
</script>

<svelte:window on:resize={handleResize} bind:scrollY />
