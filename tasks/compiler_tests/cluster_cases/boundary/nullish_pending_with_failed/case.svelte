<script>
	let pending = null;
	let failed = () => {};
</script>
<svelte:boundary {pending} {failed}>hi</svelte:boundary>
