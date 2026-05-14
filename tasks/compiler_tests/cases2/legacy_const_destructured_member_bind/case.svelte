<script>
    import Child from './Child.svelte';

    function makePayload() {
        return { payoffLazy: { data: null } };
    }

    const { payoffLazy } = makePayload();
</script>

<Child bind:payoffStore={payoffLazy.data} />
