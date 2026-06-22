<script>
  import Child from "./Child.svelte";
</script>

<div class="wrap">
  <Child>
    <svelte:fragment slot="x">
      <div class="mid"><p>hi</p></div>
    </svelte:fragment>
  </Child>
</div>

<style>
  .wrap p { color: red; }
  .mid p { color: blue; }
</style>
