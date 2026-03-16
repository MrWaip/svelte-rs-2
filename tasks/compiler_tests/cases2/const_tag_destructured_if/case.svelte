<script>
  let { show, point } = $props();
</script>

{#if show}
  {@const {x, y} = point}
  <p>{x}</p>
{/if}
