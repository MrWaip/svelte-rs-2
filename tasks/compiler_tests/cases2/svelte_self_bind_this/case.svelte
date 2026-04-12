<script>
	let ref;
</script>

{#if true}
	<svelte:self bind:this={ref} answer={42} />
{/if}
