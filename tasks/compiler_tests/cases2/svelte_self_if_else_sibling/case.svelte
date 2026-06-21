<script>
	let count = 1;
</script>

{#if count > 0}
	<p>a</p>
{:else}
	<p>b</p>
	<svelte:self />
{/if}
