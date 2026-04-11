<script>
	let { theme = 'primary' } = $props();
</script>

<button class="ui-button {theme}">x</button>

<style>
	.ui-button.primary {
		background: #0070f3;
	}
</style>
