<script>
  let { count = 0 } = $props();

  function increment() {
    count++;
  }
</script>

<p>{count}</p>
