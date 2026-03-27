<script>
	let count = $state(0);
</script>

<p>{$effect.pending()}</p>

{#if $effect.pending()}
	<p>Loading {count}</p>
{:else}
	<p>Done</p>
{/if}
