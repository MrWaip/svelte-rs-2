<script>
  let { props } = $props();
</script>

{#if true}
  {@const { x } = props}
  <p>{x}</p>
{/if}
