<svelte:options customElement={{ tag: 'x-foo' }} />
<script>
let { x, ...rest } = $props();
</script>
<p>{x}</p>
