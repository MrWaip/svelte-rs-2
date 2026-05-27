<script>
  import Outer from "./Outer.svelte";
  export let items = [];
</script>
<Outer>
  <div slot="cell" let:index let:style {style}>
    {@const item = items[index]}
    {item}
  </div>
</Outer>
