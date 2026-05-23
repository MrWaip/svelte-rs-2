<svelte:options runes={false} />
<script>
  import { writable } from 'svelte/store';
  let value = writable('');
  $: x = $value;
</script>
<input bind:value={value}>
<p>{x}</p>
