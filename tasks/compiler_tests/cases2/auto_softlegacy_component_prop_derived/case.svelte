<script>
    import Child from './Child.svelte';
    function compute() {
        return 1;
    }
</script>

<Child track={compute()} />
