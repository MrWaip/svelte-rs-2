<script>
	let { getText = (item) => String(item), getUrl } = $props();
</script>

<span>{getText(1)}{getUrl}</span>
