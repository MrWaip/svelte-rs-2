<script>
    let counter = $state(0);
    let active = $state(false);

    function getHandler() {
        return () => counter++;
    }
</script>

<div
    class:active={active}
    class:big={counter > 10}
    style:color={active ? "red" : "blue"}
    onfocus={getHandler()}
>
    content
</div>