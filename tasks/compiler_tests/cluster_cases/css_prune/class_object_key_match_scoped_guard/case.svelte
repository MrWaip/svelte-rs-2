<script>
	let { c } = $props();
</script>

<div class={{ foo: c }}>x</div>

<style>
	.foo { color: red; }
</style>
