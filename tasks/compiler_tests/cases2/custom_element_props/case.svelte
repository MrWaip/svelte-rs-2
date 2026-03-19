<svelte:options customElement="my-element" />
<script>
  let { count, label } = $props();
</script>
<p>{label}: {count}</p>
