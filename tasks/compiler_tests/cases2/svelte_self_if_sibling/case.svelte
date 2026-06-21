<script>
	let count = 1;
</script>

{#if count > 0}
	<p>ok</p>
	<svelte:self />
{/if}
