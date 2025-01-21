<script>
    let title = $state(10);
    let flag = $state();

    onMount(() => {
        title = 20;
    });
</script><div>{title}</div><div {flag}></div>