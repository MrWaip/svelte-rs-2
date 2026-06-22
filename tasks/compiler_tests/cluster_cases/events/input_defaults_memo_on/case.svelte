<script>
  import { createBubbler } from 'svelte/legacy';
  export let checked = false;
  const bubbler = createBubbler();
</script>
<input bind:checked type="checkbox" on:change={bubbler('change')} />
