<svelte:boundary>
	<svelte:boundary>
		{#snippet failed()}y{/snippet}
	</svelte:boundary>
	{#snippet other()}x{/snippet}
	{#snippet failed()}z{/snippet}
</svelte:boundary>
