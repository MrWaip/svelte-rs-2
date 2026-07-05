<script>
	let handler = $state((e) => console.error(e));
</script>

<svelte:boundary onerror={handler}>x</svelte:boundary>
