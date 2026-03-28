<script>
	let x = 1;
	let data = await fetch('/api');
	let y = data.value;
</script>

<p>{y}</p>
