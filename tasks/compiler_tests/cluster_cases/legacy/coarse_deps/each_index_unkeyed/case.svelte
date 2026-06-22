<script>
    export let items;
</script>

{#each items as item, idx}
    {@const label = `${idx}:${item.name}`}
    <p>{label}</p>
{/each}
