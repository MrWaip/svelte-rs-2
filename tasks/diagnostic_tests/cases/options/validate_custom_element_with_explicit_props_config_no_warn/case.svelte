<svelte:options customElement={{ tag: 'x-foo', props: { x: { reflect: true, type: 'Number' } } }} />
<script>
const props = $props();
</script>
<p>{props.x}</p>
