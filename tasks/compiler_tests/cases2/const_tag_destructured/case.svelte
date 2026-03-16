<script>
  let { items } = $props();
</script>

{#each items as item}
  {@const {x, y} = item}
  <p>{x}</p>
{/each}
