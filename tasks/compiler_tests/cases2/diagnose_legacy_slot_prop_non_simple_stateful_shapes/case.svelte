<script>
    export let a = 0;
    export let b = 0;
    export let flag = false;
</script>

<slot
    sum={a + b}
    neg={!flag}
    both={a && b}
    cond={flag ? a : b}
/>
