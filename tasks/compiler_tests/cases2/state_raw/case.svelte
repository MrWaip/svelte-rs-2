<script>
    let count = $state.raw(0);
    let items = $state.raw([1, 2, 3]);
    let empty = $state.raw();
    let readonly_obj = $state.raw({ x: 1 });

    count = 10;
    count += 5;
    items = [4, 5, 6];
</script>

<div>{count}</div>
