<svelte:options runes={false} />
<script>
    const meta = { items: [1, 2, 3] };
</script>

{#each meta.items as item}
    <span>{item}</span>
{/each}
