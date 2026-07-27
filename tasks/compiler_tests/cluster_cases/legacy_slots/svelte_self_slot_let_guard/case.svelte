<script>
	export let depth = 0;
</script>

{#if depth > 0}
	<svelte:self let:item let:index depth={depth - 1}>
		<p>{item} {index}</p>
	</svelte:self>
{/if}
