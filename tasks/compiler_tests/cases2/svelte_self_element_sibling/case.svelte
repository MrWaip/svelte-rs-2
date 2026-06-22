<script>
	let count = 1;
</script>

{#if count > 0}
	<div><p>ok</p><svelte:self /></div>
{/if}
