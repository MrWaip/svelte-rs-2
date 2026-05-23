<svelte:options runes={false} />
<script>
    export let items = [1, 2, 3];
    function sideFx() {}
</script>

{#each (sideFx(), items) as item}
    <span>{item}</span>
{/each}
