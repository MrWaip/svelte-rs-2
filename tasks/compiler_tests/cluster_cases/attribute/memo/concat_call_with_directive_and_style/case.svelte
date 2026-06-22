<script>
  export let status = 'neutral';
  export let disabled = false;
  function classify(s) { return s + '-x'; }
  function widthOf(s) { return s.length; }
</script>
<div class="slider {classify(status) || ''}" class:disabled style="width: {widthOf(status)}px"></div>