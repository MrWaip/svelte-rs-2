<script>
	const noop = () => {};
</script>

<button onclick={() => { const n = $state(5); noop(n); }}>go</button>
