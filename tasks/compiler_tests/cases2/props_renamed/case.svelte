<script>
  let { foo: localFoo = 'default', bar: localBar } = $props();
</script>

<p>{localFoo} {localBar}</p>
