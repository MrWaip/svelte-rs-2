<script>
  function makeValue() {
    return 42;
  }
  const value = $derived(makeValue());
</script>
<span>{value}</span>
