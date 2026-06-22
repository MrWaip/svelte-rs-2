<script>
  import Child from "./Child.svelte";
</script>

<div class="wrap">
  <Child>
    <svelte:fragment slot="x">
      <p>hi</p>
    </svelte:fragment>
  </Child>
</div>

<style>
  .wrap > p { color: red; }
</style>
