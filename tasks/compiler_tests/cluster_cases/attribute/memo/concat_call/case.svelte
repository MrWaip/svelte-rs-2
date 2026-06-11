<script>
  export let status = 'neutral';
  function classify(s) { return s + '-x'; }
</script>
<div class="slider {classify(status) || ''}"></div>