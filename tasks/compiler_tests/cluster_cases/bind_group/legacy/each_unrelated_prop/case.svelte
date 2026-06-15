<svelte:options runes={false} />
<script>
	export let selected;
	export let values;
</script>

{#each values as value}
	<input type="checkbox" value="{value}" bind:group={selected} />
{/each}
