<script>
  let count = $state(0);
  function make() { return () => count++; }
</script>
<div onclick={make()} title={count}></div>
