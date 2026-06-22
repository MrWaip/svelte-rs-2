<script>
    import Child from "./Child.svelte";

    let value = "";
</script>

<Child bind:value={value} />
