<script>
	let pending = null;
</script>
<svelte:boundary {pending}>
	{#snippet pending()}wait{/snippet}
	hi
</svelte:boundary>
