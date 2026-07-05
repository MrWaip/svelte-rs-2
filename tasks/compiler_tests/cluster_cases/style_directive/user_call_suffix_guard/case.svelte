<script>
	let x = $state(0);
	function foo(n) {
		return n + 1;
	}
</script>

<button onclick={() => x++}>go</button>
<div style:left="{foo(x)}px"></div>
