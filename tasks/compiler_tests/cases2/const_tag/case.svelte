<script>
  let { items } = $props();
</script>
{#each items as item}
  {@const doubled = item * 2}
  <p>{doubled}</p>
{/each}
