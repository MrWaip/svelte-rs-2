<script>
	let p = $state(null);
</script>
<button onclick={() => (p = new Promise(() => {}))}>{p}</button>
