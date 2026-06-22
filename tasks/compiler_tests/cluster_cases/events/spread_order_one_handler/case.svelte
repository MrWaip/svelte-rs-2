<script>
  import { createBubbler } from 'svelte/legacy';
  export let rest = {};
  const bubbler = createBubbler();
</script>
<div {...rest} on:click={bubbler('click')}></div>
