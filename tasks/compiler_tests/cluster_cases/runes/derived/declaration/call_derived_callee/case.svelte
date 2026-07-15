<script>
  let { config } = $props();
  const { makeStore } = $derived(config);
  const entries = $derived(makeStore());
</script>
<span>{entries.x}</span>
