<script>
	export let items;
</script>

<svelte:head>
	{#each items as i}
		<title>{i}</title>
	{/each}
</svelte:head>
