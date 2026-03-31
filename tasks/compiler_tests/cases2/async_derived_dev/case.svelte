<script>
	let data = $derived(await fetch('/api'));
</script>

<p>{data}</p>
