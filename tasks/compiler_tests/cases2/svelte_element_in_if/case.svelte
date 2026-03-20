<script>
	let show = $state(true);
	let tag = $state('div');
</script>

{#if show}
	<svelte:element this={tag}>content</svelte:element>
{/if}
