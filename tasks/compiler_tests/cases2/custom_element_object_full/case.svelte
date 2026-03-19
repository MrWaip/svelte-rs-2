<svelte:options customElement={{ tag: "my-component", props: { count: { type: 'Number', reflect: true } }, shadow: "open" }} />
<script>
  let { count = 0 } = $props();
</script>
<p>{count}</p>
