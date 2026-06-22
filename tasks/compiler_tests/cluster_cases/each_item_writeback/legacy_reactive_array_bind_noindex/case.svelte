<svelte:options runes={false} />
<script>
	let list = [{}];
</script>

{#each list as item}
	<Child bind:value={item} />
{/each}
