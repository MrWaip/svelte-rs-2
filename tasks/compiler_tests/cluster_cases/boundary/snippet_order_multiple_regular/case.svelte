<svelte:boundary>
	{#snippet a()}1{/snippet}
	<svelte:boundary>
		{#snippet failed()}y{/snippet}
	</svelte:boundary>
	{#snippet b()}2{/snippet}
	{#snippet failed()}z{/snippet}
</svelte:boundary>
