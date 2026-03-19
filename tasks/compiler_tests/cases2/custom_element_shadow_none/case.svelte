<svelte:options customElement={{ tag: "my-element", shadow: "none" }} />
<script>
  let { name } = $props();
</script>
<p>{name}</p>
