{#snippet pending()}loading{/snippet}
<svelte:boundary {pending}>
	{@const data = await Promise.resolve("d")}
	{data}
</svelte:boundary>
