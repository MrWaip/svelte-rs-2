<svelte:options customElement={{ tag: "my-element", shadow: "open" }} />
<script>
  let { name } = $props();
</script>
<p>{name}</p>
