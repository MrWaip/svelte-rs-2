<style>
  div {
    color: blue;
  }
</style>

<div>
  <style>
    .nested {
      color: red;
    }
  </style>

  <p class="nested">inside div</p>
</div>

{#if true}
  <style>
    span {
      color: green;
    }
  </style>
{/if}
