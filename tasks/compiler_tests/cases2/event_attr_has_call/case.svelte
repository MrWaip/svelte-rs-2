<script>
	function getHandler() {
		return () => console.log('clicked');
	}
</script>

<button onclick={getHandler()}>Click</button>
