<script>
    let counter = $state(0);
    let active = $state(false);

    function getHandler() {
        return () => counter++;
    }
</script>

<div
    class:active
    class={{ big: counter > 10 }}
    onfocus={getHandler()}
>
    content
</div>
