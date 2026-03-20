<script>
	let items = $state(['a', 'b', 'c']);
	let selected = $state([]);
</script>

{#each items as item}
	<input type="checkbox" bind:group={selected} value={item}>
{/each}
