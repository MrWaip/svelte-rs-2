<script>
    let title = $state(10);
    let flag = $state();
    let flag2 = $state();
    let value = $state("text");

    onMount(() => {
        title = 20;
        window.id = title;
        flag2 = title;
        map(title);
    });

    function map(value, off = title) {
        return value;
    }

    value += 1234;
    value -= 4000;
    value *= 2;
    value &&= fallback;
    value = "";

    const obj = { title: title, title };
</script>

<div>{title}</div>

<div {flag}>
    {flag2}
</div>
