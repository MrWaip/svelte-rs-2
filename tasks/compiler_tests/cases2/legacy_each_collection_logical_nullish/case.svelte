<svelte:options runes={false} />
<script>
    export let items = null;
</script>

{#each items ?? [] as item}
    <span>{item}</span>
{/each}
