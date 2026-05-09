<script>
    import Child from './Child.svelte';
    let v = $state();
    function onx() {}
</script>

<Child a="1" bind:value={v} onx={onx} b="2" />
