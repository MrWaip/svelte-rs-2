<script>
	let handler = $state((error) => console.error(error));
</script>

<svelte:boundary onerror={handler}>
	<p>content</p>
</svelte:boundary>
