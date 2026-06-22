<script>
  import { createBubbler } from 'svelte/legacy';
  export let rest = {};
  const bubbler = createBubbler();
  function action(node) {}
</script>
<div {...rest} use:action on:click={bubbler('click')}></div>
