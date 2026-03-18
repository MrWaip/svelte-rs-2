<script>
	let show = $state(true);
</script>

{#if show}
	<svelte:boundary>
		<p>guarded</p>

		{#snippet failed(error)}
			<p>{error.message}</p>
		{/snippet}
	</svelte:boundary>
{/if}
