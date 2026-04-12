<script>
	let { value: local = { stats: { count: 0 } } } = $props();
	let key = 'count';

	function bump() {
		local.stats[key]++;
	}
</script>
