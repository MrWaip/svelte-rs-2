<script>
	export let a;
	export let b;
</script>

<svelte:head>
	{#if a}
		{#if b}
			<title>deep</title>
		{/if}
	{/if}
</svelte:head>
