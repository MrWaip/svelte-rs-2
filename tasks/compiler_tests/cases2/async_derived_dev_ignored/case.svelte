<script>
	// svelte-ignore await_waterfall
	let data = $derived(await fetch('/api'));
</script>

<p>{data}</p>
