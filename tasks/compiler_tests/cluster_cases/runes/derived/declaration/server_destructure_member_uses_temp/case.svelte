<script>
	let { confirmStore } = $props();
	const { phone, rate } = $derived(confirmStore.data);
</script>
<span>{phone}{rate}</span>
