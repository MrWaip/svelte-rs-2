<script>
  import { createBubbler } from 'svelte/legacy';
  const bubbler = createBubbler();
</script>
<input type="text" on:click={bubbler('click')} />
