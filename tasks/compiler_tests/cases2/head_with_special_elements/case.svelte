<script>
    let count = $state(0);

    function handleScroll() {
        count++;
    }
</script>

<svelte:head>
    <title>Count: {count}</title>
    <meta name="description" content="test">
</svelte:head>

<svelte:window onscroll={handleScroll} />

<div>Count: {count}</div>
