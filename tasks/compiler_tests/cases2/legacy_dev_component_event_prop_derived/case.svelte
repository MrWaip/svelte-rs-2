<script>
    import Child from "./Child.svelte";

    function handleClick() {}
    function getHandler() {
        return handleClick;
    }
</script>

<Child onclick={getHandler()} />
