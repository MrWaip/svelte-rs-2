<script>
    let count = 0;

    function bump() {
        count += 1;
    }
</script>

<svelte:head>
    <title>Count: {count}</title>
</svelte:head>

<button onclick={bump}>+</button>
