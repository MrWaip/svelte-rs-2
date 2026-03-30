<script>
	let title = $state("/api");
</script>

<svelte:head>
	<title>{await fetch(title)}</title>
</svelte:head>
