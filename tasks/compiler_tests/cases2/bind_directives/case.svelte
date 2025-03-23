<script>
    let value = $state('');
    let name = '' ;
</script>

<input bind:value />

<input bind:value={name} />

<input bind:value={name} {...other} />

<input defaultValue="123" bind:value={name}  />