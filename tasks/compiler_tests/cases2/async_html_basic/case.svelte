<script>
	async function loadContent() {
		return '<b>hello</b>';
	}
</script>

{@html await loadContent()}
