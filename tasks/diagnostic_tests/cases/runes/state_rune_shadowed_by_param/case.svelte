<script>
	let foo = $state(0);
	function bar($state) {
		return $state(foo);
	}
</script>

<button onclick={() => foo++}>{foo}{bar((v) => v)}</button>
