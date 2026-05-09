<script>
	let { onClick, onCustom } = $props();
</script>

<svelte:window onclick={onClick} oncustom-event={onCustom} />
