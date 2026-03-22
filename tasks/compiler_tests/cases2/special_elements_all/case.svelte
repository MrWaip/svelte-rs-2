<script>
    let count = $state(0);

    function handleEvent() {
        count++;
    }

    function action(node) {
        return { destroy() {} };
    }
</script>

<svelte:head>
    <title>App: {count}</title>
    <meta name="viewport" content="width=device-width">
</svelte:head>

<svelte:window onscroll={handleEvent} />
<svelte:document onvisibilitychange={handleEvent} />
<svelte:body onmouseenter={handleEvent} use:action />

<div>
    <p>Count: {count}</p>
</div>
