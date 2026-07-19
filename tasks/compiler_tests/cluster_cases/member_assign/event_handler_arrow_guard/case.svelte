<script>
	let obj = $state({ x: null });
	let src = $state({});
</script>

<button onclick={() => (obj.x = src)}>go</button>
