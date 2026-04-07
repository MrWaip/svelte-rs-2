<svelte:options customElement="my-counter" />
<script>
  let { count: total = 0 } = $props();
</script>
<p>{total}</p>
