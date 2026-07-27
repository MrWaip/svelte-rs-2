<svelte:boundary>
	{#snippet other()}x{/snippet}
	<svelte:boundary>
		{#snippet failed()}y{/snippet}
	</svelte:boundary>
	{#snippet failed()}z{/snippet}
</svelte:boundary>
