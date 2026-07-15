<script>
	let anchorRefs = {};
	let groups = $state([]);
</script>

{#each groups as group (group.key)}
	<div bind:this={anchorRefs[group.key]}>{group.key}</div>
{/each}
