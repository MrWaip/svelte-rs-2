<script>
    let count = $state(0);
    let { row } = $props();
</script>

{@render row(count + 1)}
