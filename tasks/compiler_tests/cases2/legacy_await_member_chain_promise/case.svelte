<svelte:options runes={false} />
<script>
    export let user = { fetch: () => Promise.resolve(null) };
</script>

{#await user.fetch() then v}
    <span>{v}</span>
{/await}
