<svelte:options runes={false} />

<script>
    export let items;
    export let item;
    let target;
    function handle(e) {
        [target] = e;
    }
</script>

{handle}
{#each items as item (item.id)}
    <div>{item.id}</div>
{/each}
