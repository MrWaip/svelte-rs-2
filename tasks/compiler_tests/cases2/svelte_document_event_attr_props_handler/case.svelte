<script>
	let { onClick, onCustom } = $props();
</script>

<svelte:document onclick={onClick} oncustom-event={onCustom} />
