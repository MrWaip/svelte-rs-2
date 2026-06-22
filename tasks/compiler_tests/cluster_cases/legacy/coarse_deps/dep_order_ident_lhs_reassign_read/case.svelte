<script>
	let x = 0;
	let c = 0;
	$: x = x + c;
</script>

<input bind:value={c} />
