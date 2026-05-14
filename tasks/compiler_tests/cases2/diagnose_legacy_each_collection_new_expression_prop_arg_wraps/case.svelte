<svelte:options runes={false} />

<script>
    export let size;
</script>

{#each new Array(size).fill(null) as _, i (i)}
    <div>{i}</div>
{/each}
