<svelte:options runes={false} />
<script>
  import Sticker from './Sticker.svelte';
  export let items = [];
  const color = (x) => x.color ?? 'red';
</script>

{#each items as item}
  <Sticker {...item} --bg={color(item)} />
{/each}
