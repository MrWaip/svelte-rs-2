<script>
  export let status = 'neutral';
  function classify(s) { return s + '-x'; }
</script>
<div class={classify(status)}></div>