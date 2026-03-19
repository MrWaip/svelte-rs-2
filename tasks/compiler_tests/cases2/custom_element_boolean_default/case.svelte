<svelte:options customElement="my-toggle" />
<script>
  let { active = false } = $props();
</script>
<p>{active}</p>
