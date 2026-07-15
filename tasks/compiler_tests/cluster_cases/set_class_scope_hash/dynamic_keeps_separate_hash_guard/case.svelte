<script>
	let { cls, on } = $props();
</script>

<div class={cls} class:active={on}>a</div>

<style>
	.active { color: blue; }
</style>
