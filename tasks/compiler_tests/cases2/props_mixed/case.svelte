<script>
  let { a, b = 10, c = $bindable(), ...rest } = $props();
</script>

<p>{a}</p>
<p>{b}</p>
<p>{c}</p>
