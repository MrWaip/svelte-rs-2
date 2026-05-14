<script>
    function makePair() { return [1, 2]; }
    const pair = makePair();
    $: [a, b] = pair;
</script>

{a}{b}
