<svelte:options runes={false} />
<script>
    export let count = 3;
</script>

{#each Array.from({ length: count }) as _}
    <span></span>
{/each}
