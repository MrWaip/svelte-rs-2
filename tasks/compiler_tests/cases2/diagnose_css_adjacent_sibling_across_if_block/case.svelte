<script>
  let { x, y } = $props();
</script>

<div class="a"></div>
{#if x}
  <div class="b"></div>
{/if}
{#if y}
  <div class="c"></div>
{/if}

<style>
  .a + .c,
  .b + .c {
    color: red;
  }
</style>
