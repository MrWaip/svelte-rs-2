<script>
    let size = $state(1);
    let arr = $state([]);
    function joinClasses(a) {
        return a.join(' ');
    }
</script>

<div {...{ id: "x" }} class="size_{size} {joinClasses(arr)}" />
