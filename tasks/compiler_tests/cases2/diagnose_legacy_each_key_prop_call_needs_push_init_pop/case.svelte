<svelte:options runes={false} />
<script>
    export let getKey;
</script>

{#each [1, 2, 3] as item (getKey())}
    <span>{item}</span>
{/each}
