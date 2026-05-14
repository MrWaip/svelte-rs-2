<script>
    let { flag } = $props();
    let invalid = $derived(Boolean(flag));

    function reset() {
        invalid = false;
    }
</script>

<button onclick={reset}>{invalid}</button>
