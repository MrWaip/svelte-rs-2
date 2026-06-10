<script>
    import Child from "./Child.svelte";

    let ref;
</script>

<Child bind:this={ref} />
