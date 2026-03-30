<script>
	let response = await fetch('/api');
</script>

<div title={await response.text()}></div>
