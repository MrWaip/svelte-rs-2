<script>
	let ops = $state([{ args: [{ value: [], options: [{ value: "a" }] }], id: 1 }]);
</script>

{#each ops as { args, id } (id)}
	{#each args as arg}
		{#each arg.options as { value }}
			<input type="checkbox" bind:group={arg.value} {value} />
		{/each}
	{/each}
{/each}
