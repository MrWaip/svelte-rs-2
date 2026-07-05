<script>
	let count = $state(0);
	function fn(x) { return x; }
</script>
<p>v {fn(count)}</p>
<button onclick={() => count++}>+</button>
