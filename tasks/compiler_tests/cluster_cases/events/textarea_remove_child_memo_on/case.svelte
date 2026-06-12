<script>
  import { createBubbler } from 'svelte/legacy';
  export let value = '';
  const bubbler = createBubbler();
</script>
<textarea bind:value on:input={bubbler('input')}></textarea>
