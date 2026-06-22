<script>
  import Comp from "./Comp.svelte";
  export let value;
  function fn(v) { return v + 1; }
</script>
<Comp id="{fn(value)}" />