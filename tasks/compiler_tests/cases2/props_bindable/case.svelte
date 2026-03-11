<script>
  let { value = $bindable(), count = $bindable(0) } = $props();
</script>

<p>{value}</p>
<p>{count}</p>
