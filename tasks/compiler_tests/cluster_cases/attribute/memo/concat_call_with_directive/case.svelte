<script>
  export let status = 'neutral';
  export let disabled = false;
  function classify(s) { return s + '-x'; }
</script>
<div class="slider {classify(status) || ''}" class:disabled></div>