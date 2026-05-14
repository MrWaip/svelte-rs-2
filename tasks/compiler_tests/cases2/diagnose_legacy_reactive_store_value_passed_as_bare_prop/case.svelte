<script>
    import Child from './Child.svelte';
    export let source;

    $: store = source;
    $: doubled = $store * 2;
</script>

<Child value={store} other={doubled} />
