<script>
	let { loading, items } = $props();
</script>

{#if loading}
	{#each items as item}<span>{item}</span>{/each}
{:else}
	{#each items as item}<div>{item}</div>{/each}
{/if}
