<script>
	const noop = () => {};
</script>

<button onclick={() => { if (noop) { const s = $state({ x: 1 }); noop(s); } }}>go</button>
