<script>
	let count = $state(0);

	function action(node) {
		return {
			update(count) {
				console.log('update', this.count, (this.count = count));
			},
		};
	}
</script>

<button use:action onclick={() => count++}>{count}</button>
