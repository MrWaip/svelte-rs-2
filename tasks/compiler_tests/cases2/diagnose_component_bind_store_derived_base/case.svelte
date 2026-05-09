<script>
    import Child from './Child.svelte';
    function makeStore() { return { search: null }; }
    let outer = $state();
    const { search } = $derived(makeStore(outer));
</script>

<Child bind:value={$search} />
