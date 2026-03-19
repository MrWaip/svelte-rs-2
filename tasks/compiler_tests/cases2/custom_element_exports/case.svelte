<svelte:options customElement="my-widget" />
<script>
  let { value = 0 } = $props();
  export function reset() { value = 0; }
</script>
<p>{value}</p>
