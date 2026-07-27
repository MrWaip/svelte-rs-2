<script>
	let obj = $state({ x: 0 });
	let value = $state(0);
</script>

<input bind:value={() => value, (v) => (obj.x = { n: v })} />
