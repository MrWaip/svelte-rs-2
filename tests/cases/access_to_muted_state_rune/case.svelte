<script>
    let title = $state(10);
    let flag = $state();

    onMount(() => {
        title = 20;
        map(title);
    });

    function map(value, off = title) {
        return value;
    }

    const obj = { title: title, title };
</script><div>{title}</div><div {flag}></div>