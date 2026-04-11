<svelte:options customElement={{ tag: 'x-foo' }} />
<script>
let { x, y } = $props();
</script>
<p>{x}{y}</p>
