<script>
  import Child from "./Child.svelte";
</script>

<div class="wrap">
  <Child>
    <p>hi</p>
  </Child>
</div>

<style>
  .wrap:has(p) { color: red; }
</style>
