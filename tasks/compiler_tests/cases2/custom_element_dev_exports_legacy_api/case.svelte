<svelte:options customElement="my-el" />
<script>
  let { x = 0, ...rest } = $props();
  export const VERSION = "1";
  export function helper() {}
</script>
<p>{x}</p>
