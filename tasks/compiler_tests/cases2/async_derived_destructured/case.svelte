<script>
	let url = $state('/api');
	let { data, meta } = $derived(await fetch(url).then((r) => r.json()));
</script>

<p>{data}-{meta}</p>
