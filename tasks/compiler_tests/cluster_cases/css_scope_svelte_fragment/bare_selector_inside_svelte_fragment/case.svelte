<script>
  import Child from "./Child.svelte";
</script>

<Child>
  <svelte:fragment slot="x">
    <p>hi</p>
  </svelte:fragment>
</Child>

<style>
  p { color: red; }
</style>
