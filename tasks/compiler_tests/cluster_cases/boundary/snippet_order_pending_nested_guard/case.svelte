<svelte:boundary>
	{#snippet other()}x{/snippet}
	<svelte:boundary>
		{#snippet pending()}p{/snippet}
	</svelte:boundary>
	{#snippet failed()}z{/snippet}
</svelte:boundary>
