<script>
    let selected = $state([]);
</script>
<input type="checkbox" bind:group={(() => selected, (next) => selected = next, selected)}>
