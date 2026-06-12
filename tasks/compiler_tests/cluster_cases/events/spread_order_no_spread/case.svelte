<script>
  import { createBubbler } from 'svelte/legacy';
  export let title = '';
  const bubbler = createBubbler();
</script>
<div {title} on:click={bubbler('click')}></div>
