<svelte:options customElement={{ tag: "my-counter", props: { count: { type: 'Number', reflect: true }, label: { attribute: 'data-label' } } }} />
<script>
  let { count, label } = $props();
</script>
<p>{label}: {count}</p>
