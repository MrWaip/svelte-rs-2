<script>
	function handleScroll() {
		console.log('scrolled');
	}
</script>

<svelte:window onscroll={handleScroll} />
