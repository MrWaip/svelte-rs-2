<script>
	let data = await fetch('/api');
	let handler = $state(data.handler);
</script>

<div {@attach handler}>hello</div>
