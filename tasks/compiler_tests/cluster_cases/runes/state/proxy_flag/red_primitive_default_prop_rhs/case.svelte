<script>
	let { flag = false } = $props();
	let value = $state(0);
	function apply() {
		value = flag;
	}
</script>

<button onclick={apply}>{value}</button>
