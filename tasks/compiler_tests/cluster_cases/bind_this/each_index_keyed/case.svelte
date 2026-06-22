<script>
	export let data = [{ id: '1' }];
	export let refs = [];
</script>

{#each data as { id }, index (id)}
	<div bind:this={refs[index]}>{id}</div>
{/each}
