<script>
	let active = $state(false);
	let big = $state(false);
</script>

<div class={{ active, big }} class:extra={active}>content</div>

<style>
	div { color: black; }
	.active { color: red; }
</style>
