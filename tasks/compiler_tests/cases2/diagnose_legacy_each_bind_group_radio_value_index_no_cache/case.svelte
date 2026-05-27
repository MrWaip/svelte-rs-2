<svelte:options runes={false} />
<script>
	export let active;
</script>

{#each [1, 2, 3] as _, index}
	<input type="radio" value={index} bind:group={active} />
{/each}
