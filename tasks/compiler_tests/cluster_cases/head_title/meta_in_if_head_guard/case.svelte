<script>
	export let condition;
</script>

<svelte:head>
	{#if condition}
		<meta id="m" name="x" content="y"/>
	{/if}
</svelte:head>
