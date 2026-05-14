<script>
    import Child from './Child.svelte';
    export let cond = false;
    function a() {}
    function b() {}
    $: handler = cond ? a : b;
</script>

<Child on:click={handler} />
