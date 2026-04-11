<script>
    let value = $state('');
    let getter = () => value;
</script>
<input bind:value={getter()}>
