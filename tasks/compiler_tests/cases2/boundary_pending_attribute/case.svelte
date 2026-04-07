<script>
	function pending($$anchor) {
		console.log($$anchor);
	}
</script>

<svelte:boundary pending={pending}>
	<p>content</p>
</svelte:boundary>
