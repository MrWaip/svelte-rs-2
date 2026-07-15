<script>
	let { tag } = $props();
</script>

<div>Текст <svelte:element this={tag}>жирный</svelte:element></div>
