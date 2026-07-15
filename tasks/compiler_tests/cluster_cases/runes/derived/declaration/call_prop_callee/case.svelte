<script>
  let { getData } = $props();
  const value = $derived(getData());
</script>
<span>{value}</span>
