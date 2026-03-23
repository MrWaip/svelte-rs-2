<script>
    let counter = $state(0);
    let color = $state("red");

    function getHandler() {
        return () => { counter++; color = "blue"; };
    }
</script>

<div
    class:active={counter > 5}
    class:big={counter > 10}
    style:color={color}
    onfocus={getHandler()}
>
    content
</div>