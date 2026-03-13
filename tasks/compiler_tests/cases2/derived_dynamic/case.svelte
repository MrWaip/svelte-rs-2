<script>
  let count = $state(0);
  let doubled = $derived(count * 2);
  function increment() { count++; }
</script>
<button onclick={increment}>{doubled}</button>
