<script>
  let src = $state('');
  let cond = $state(false);
  function on_load() {}
</script>

<svelte:head>
  {#if cond}
    <script async {src} onload={on_load}></script>
  {/if}
</svelte:head>
