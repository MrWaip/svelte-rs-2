<script>
  export let status = 'neutral';
  function classify(s) { return s + '-x'; }
  function widthOf(s) { return s.length; }
</script>
<div class="slider {classify(status) || ''}" style="width: {widthOf(status)}px"></div>