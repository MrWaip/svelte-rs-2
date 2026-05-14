<script>
    let { cond, a, b } = $props();
</script>

{@render (cond ? a : b)()}
