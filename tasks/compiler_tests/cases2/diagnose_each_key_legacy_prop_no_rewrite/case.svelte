<svelte:options runes={false} />
<script>
	export let items = [];
	export let prefix = '';
</script>

{#each items as item, i (prefix + i)}
	<p>{item}</p>
{/each}
