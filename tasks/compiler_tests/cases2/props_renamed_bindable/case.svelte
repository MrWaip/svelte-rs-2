<script>
  let { value: localVal = $bindable('fallback') } = $props();
</script>

<p>{localVal}</p>
