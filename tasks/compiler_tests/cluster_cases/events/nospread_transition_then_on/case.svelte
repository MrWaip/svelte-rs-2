<script>
  import { createBubbler } from 'svelte/legacy';
  import { slide } from 'svelte/transition';
  const bubbler = createBubbler();
</script>
<div transition:slide on:click={bubbler('click')}></div>
