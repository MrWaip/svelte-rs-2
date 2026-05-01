<script>
	let xs = $state([0]);

	async function index() {
		return 0;
	}
</script>

<input bind:value={xs[await index()]}>
