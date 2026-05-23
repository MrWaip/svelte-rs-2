<svelte:options runes={false} />
<script>
    export let foo;
    function items() {
        return [1, 2, 3];
    }
</script>

{#each items() as item}
    <div>{item}</div>
{/each}
