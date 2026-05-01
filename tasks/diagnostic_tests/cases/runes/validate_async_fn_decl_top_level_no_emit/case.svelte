<script>
	async function load() {
		return await fetch('/x');
	}
</script>

<p>{load.name}</p>
