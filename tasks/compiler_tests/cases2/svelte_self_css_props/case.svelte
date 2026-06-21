<script>
	let count = 1;
	let color = 'red';
</script>

{#if count > 0}
	<svelte:self --my-color={color} />
{/if}
