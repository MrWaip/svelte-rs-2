<script>
    class Counter {
        value = $state(0);
    }

    let c = new Counter();
</script>

<button on:click={() => (c.value = c.value + 1)}>{c.value}</button>
