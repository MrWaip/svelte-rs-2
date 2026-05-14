<script>
  let props = $props();
</script>
{#if true}
  {@const { onClick } = props}
  <button onclick={onClick}>x</button>
{/if}
