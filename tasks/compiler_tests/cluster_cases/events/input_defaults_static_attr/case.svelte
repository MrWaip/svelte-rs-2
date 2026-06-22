<script>
  import { createBubbler } from 'svelte/legacy';
  export let checked = false;
  const bubbler = createBubbler();
  const u = 'http://x';
</script>
<input bind:checked formaction={u} type="checkbox" on:change={bubbler('change')} />
