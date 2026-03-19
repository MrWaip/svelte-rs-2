<svelte:options customElement={{ tag: "my-element", extend: customElementWrapper }} />
<script>
  import customElementWrapper from './wrapper.js';
  let { name } = $props();
</script>
<p>{name}</p>
