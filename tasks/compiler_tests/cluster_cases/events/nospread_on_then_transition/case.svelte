<script>
  import { createBubbler } from 'svelte/legacy';
  import { slide } from 'svelte/transition';
  const bubbler = createBubbler();
</script>
<div on:click={bubbler('click')} transition:slide></div>
