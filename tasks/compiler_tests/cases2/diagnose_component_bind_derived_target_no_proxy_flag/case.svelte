<script>
    import Child from './Child.svelte';
    let base = $state(false);
    let derivedFlag = $derived(base);
</script>

<Child bind:value={derivedFlag} />
