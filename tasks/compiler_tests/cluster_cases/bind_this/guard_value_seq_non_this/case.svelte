<script>
	let x = $state(0);
</script>
<input bind:value={() => x, v => x = v} />
