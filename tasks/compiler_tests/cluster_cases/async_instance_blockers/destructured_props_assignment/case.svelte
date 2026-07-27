<script>
	let loaded = await Promise.resolve(1);
	let { name } = $props();
</script>

<p>{loaded}{name}</p>
