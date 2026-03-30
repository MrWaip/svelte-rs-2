<script>
	let response = await fetch('/api');
</script>

<p>{await response.text()}</p>
