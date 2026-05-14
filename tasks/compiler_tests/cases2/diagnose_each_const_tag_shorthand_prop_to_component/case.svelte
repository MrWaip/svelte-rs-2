<script>
  import Child from './Child.svelte';
  const items = [];
</script>

{#each items as item}
  {@const callback = () => item.id}
  <Child {callback} />
{/each}
