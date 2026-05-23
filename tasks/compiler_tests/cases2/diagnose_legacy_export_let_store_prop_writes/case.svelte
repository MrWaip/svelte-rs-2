<svelte:options runes={false} />
<script>
  export let store;

  function clear() {
    $store = undefined;
  }

  function bump() {
    $store.x = 1;
  }
</script>

<button on:click={clear}>{$store?.x}</button>
<button on:click={bump}>b</button>
