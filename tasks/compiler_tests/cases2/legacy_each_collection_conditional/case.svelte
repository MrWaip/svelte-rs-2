<svelte:options runes={false} />
<script>
    export let cond = true;
    export let a = [1];
    export let b = [2];
</script>

{#each cond ? a : b as item}
    <span>{item}</span>
{/each}
