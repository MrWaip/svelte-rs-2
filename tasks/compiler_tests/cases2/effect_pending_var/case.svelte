<script>
	let count = $state(0);
</script>

{#if $effect.pending()}
	<p>Loading {count}</p>
{:else}
	<p>Done</p>
{/if}
