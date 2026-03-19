<svelte:options customElement={{ shadow: "open" }} />
<script>
  let { name } = $props();
</script>
<p>{name}</p>
