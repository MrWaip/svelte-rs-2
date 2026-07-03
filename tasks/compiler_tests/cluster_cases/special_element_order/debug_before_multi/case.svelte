<script>
	let count = $state(0);
</script>

{@debug count}
<span>x</span>
<button onclick={() => count++}>{count}</button>
