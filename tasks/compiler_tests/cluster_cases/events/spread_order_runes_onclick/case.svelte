<script>
  let { rest } = $props();
  function f() { return () => {}; }
</script>
<div {...rest} onclick={f()}></div>
