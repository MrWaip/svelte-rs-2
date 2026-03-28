<script>
	let url = $state('/api');
	let data = $derived(await fetch(url));
</script>

<p>{data}</p>
