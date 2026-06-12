<script>
  import { createBubbler } from 'svelte/legacy';
  const bubbler = createBubbler();
  function action(node) {}
</script>
<div use:action on:click={bubbler('click')}></div>
