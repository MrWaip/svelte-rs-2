<script>
	let response = await fetch('/api');
</script>

<p>Hello {await response.text()}!</p>
