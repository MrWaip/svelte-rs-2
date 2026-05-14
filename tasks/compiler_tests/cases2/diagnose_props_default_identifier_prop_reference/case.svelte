<script>
  let { onClose, onSubmit = onClose } = $props();
</script>

<button onclick={onSubmit}>x</button>
