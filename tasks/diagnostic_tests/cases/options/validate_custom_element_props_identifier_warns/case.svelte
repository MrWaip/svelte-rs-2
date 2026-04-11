<svelte:options customElement={{ tag: 'x-foo' }} />
<script>
const props = $props();
</script>
<p>{props.x}</p>
