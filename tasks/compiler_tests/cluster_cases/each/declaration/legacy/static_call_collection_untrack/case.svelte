<svelte:options runes={false} />

<script>
	export let x;
</script>

{#each Array(10).fill(null) as _, i}
	<span>{i}{x}</span>
{/each}
