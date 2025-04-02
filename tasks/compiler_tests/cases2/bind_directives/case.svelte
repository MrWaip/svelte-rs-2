<script>
    let value = $state('');
    let name = '' ;
</script>

<input bind:value />

<input bind:value={name} />

<!--
   todo. need to implement proxy marker when assign to rune
    
<input bind:value={
    () => value,
    (newValue) => value = newValue
} />
-->

<input bind:value={name}  />