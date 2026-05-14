<svelte:options runes={false} />

<script>
  import Child from './Child.svelte';
  const item = getItem();
</script>

{#if item}
  {@const { title, status } = item}
  <Child {status}>{title}</Child>
{/if}
