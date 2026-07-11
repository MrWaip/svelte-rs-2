<script>
	let { source } = $props();
	const { phone, rate } = $derived(source);
</script>
<span>{phone}{rate}</span>
