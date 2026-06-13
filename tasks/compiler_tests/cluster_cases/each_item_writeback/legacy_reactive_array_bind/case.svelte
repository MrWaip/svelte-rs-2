<svelte:options runes={false} />
<script>
	let list = [{}];
</script>

{#each list as item, idx}
	<Child bind:value={item} />
{/each}
