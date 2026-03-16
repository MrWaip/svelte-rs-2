<script>
  let { items } = $props();
</script>

{#each items as item}
  {@const {x, y} = item}
  {@const {a, b} = item}
  <p>{x} {a}</p>
{/each}
