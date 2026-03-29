<script>
    let count = $state(0);

    function handleClick() {
        count++;
    }

    function getHandler() {
        return handleClick;
    }
</script>

<div
    onclick={handleClick}
    onscroll={handleClick}
    onclickcapture={handleClick}
    onfocus={getHandler()}
>
    {count}
</div>
