<script>
	let count = 1;
</script>

{#if count > 0}
	<svelte:self count={count - 1} />
{/if}
