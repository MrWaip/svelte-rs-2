<script>
    import Child from './Child.svelte';
    export let value;
    function build(v) { return { value: v }; }
</script>

<Child {...build(value)} />
