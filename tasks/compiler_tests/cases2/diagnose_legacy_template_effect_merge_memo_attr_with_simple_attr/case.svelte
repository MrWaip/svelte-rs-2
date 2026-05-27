<script>
    export let value = 0;
    export let label = "";
    function toPx(n) { return n + "px"; }
</script>

<div style="--w: {toPx(value)};" data-testid={label}></div>
