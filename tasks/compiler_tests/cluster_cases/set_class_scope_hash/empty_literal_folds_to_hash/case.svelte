<script>
	let { on } = $props();
</script>

<div class={''} class:active={on}>a</div>

<style>
	.active { color: blue; }
</style>
