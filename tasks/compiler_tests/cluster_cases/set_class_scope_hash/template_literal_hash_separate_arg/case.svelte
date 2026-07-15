<script>
	let { x } = $props();
</script>

<div class={`title`} class:active={x}>a</div>

<style>
	.title { color: red; }
	.active { color: blue; }
</style>
