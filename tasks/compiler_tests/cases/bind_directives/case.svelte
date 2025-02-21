
<script>
   let value = $state('');
   let checked = false;
   let group = $state();
</script>

<input bind:value />

<input bind:value={value} />

<input type="checkbox" bind:checked />

<input type="checkbox" bind:checked={checked} />

<input bind:group />

<input bind:group={group} />