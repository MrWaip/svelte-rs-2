<svelte:options runes={false} />

<script>
	export let x;
</script>

{#each [1, 2, 3] as n}
	<span>{n}{x}</span>
{/each}
