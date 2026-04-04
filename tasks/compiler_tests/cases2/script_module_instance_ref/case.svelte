<script module>
	const BASE = 'https://example.com';
</script>

<script>
	let path = $state('/home');
	let url = $derived(BASE + path);
</script>

<a href={url}>Link</a>
