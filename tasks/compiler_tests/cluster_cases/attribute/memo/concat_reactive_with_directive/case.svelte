<script>
  export let status = 'neutral';
  export let disabled = false;
</script>
<div class="slider {status || ''}" class:disabled></div>