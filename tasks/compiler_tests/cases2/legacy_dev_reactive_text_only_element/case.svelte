<script>
    let title = "x";
    let count = 0;

    function tick() {
        title = "y";
        count = 1;
    }
</script>

<p>{title}</p>
<strong>{count}</strong>
<svelte:element this="div">Dyn: {title}</svelte:element>
