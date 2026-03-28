<script>
	let x = 1;
	let data = await fetch('/api');
	let value = $state(data.text);
</script>

<input bind:value={value} />
