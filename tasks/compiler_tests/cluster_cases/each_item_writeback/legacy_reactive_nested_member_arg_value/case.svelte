<svelte:options runes={false} />
<script>
	export let pipelineOperations = [];
</script>

{#each pipelineOperations as { operation, id } (id)}
	{#each operation.args as arg}
		{#each arg.options as { value }}
			<input type="radio" bind:group={arg.value} {value} />
		{/each}
	{/each}
{/each}
