<script>
	let { keys, columns } = $props();
</script>
{#each keys as key (key)}
	{@const column = columns[key]}
	<div>{column}</div>
{/each}
