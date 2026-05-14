<svelte:options runes={false} />

<script>
    export let item;
</script>

{#key item.id}
    <div>{item.id}</div>
{/key}
