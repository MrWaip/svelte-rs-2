<script>
  let { item } = $props();
</script>

{#if item}
  {@const { name, label = 'fallback' } = item}
  <p>{name} {label}</p>
{/if}
