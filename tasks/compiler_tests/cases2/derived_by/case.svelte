<script>
  let count = $state(0);
  let doubled = $derived.by(() => count * 2);
</script>
<p>{doubled}</p>
