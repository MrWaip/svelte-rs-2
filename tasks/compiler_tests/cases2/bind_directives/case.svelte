<script>
    let value = $state('');
    let name = '' ;
</script>

<input bind:value />

<input bind:value={name} />

<input bind:value={
    () => value,
    (newValue) => value = newValue
} />

<input bind:value={name}  />