<script>
	export let items;
</script>
{#each items as _}
	<svelte:self let:item>{item}</svelte:self>
{/each}
