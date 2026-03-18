<script>
	function handler() {
		console.log('touch move');
	}
</script>

<svelte:window ontouchmove={handler} />
