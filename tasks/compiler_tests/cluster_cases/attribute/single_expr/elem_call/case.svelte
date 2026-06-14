<script>
  export let value;
  function fn(v) { return v + 1; }
</script>
<p data-x="{fn(value)}"></p>