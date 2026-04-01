<svelte:options customElement="my-element" />
<script>
  let { answer, ...rest } = $props();
  let host = $host();
</script>

<p>{answer}</p>
