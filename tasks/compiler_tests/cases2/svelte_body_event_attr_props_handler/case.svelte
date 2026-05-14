<script>
	let { onClick, onCustom } = $props();
</script>

<svelte:body onclick={onClick} oncustom-event={onCustom} />
