<script>
	let count = $state(0);
</script>

{#if $state.eager(count)}
	<p>Active</p>
{/if}
