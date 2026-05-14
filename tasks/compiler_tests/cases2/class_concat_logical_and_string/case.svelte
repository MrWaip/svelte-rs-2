<script>
	let { x = 'foo' } = $props();
	const cond = x === 'foo';
</script>

<div class="a {cond && 'b'}"></div>
