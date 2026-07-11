<script>
	export let condition;
	export let name;
</script>

<svelte:head>
	{#if condition}
		<title>Hi {name}</title>
	{/if}
</svelte:head>
