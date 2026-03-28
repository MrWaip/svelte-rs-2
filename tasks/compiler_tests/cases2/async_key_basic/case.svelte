<script>
	async function getValue() {
		return 42;
	}
</script>

{#key await getValue()}
	<p>content</p>
{/key}
