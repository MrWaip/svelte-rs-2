<script>
	function handleScroll() {
		console.log('scrolled');
	}
</script>

<svelte:window on:scroll={handleScroll} />
