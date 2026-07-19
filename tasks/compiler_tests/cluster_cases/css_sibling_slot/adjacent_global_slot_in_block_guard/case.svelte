<div>
  {#if true}<slot></slot>{/if}
  <p class="foo">foo</p>
</div>

<style>
  :global(.x) + .foo { color: green; }
</style>
