<svelte:options runes={false} />
<script>
    export let id = 1;
    async function fetchData(arg) { return arg; }
</script>

{#await fetchData(id) then v}
    <span>{v}</span>
{/await}
