<script>
	let { on } = $props();
</script>

<div class={'box'} class:active={on}>a</div>

<style>
	.box { color: red; }
	.active { color: blue; }
</style>
