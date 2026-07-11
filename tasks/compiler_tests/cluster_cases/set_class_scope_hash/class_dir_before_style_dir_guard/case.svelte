<script>
	let { on } = $props();
</script>

<div class:active={on} style:color="red">a</div>

<style>
	div { color: blue; }
	.active { color: green; }
</style>
