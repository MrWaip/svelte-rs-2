<script>
  let { x = 0 } = $props();
  function fmt(n) { return String(n); }
</script>
<div class:active={x === 0}>
  <span>{fmt(x)}</span>
</div>
