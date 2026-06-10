<script>
    export let count = 0;
    export let multiplier = 2;

    let doubled = $derived(count * multiplier);
    let summary = $derived("x:" + count);
</script>

<p>doubled={doubled}, summary={summary}</p>
