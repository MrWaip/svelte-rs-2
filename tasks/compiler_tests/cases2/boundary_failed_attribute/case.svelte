<script>
	function myFailed($$anchor, error) {
		console.log(error);
	}
</script>

<svelte:boundary failed={myFailed}>
	<p>content</p>
</svelte:boundary>
