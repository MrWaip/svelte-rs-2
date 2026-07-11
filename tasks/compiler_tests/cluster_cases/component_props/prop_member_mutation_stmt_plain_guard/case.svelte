<script>
	let { obj = $bindable() } = $props();
	function sync() {
		obj.field = obj.other;
	}
</script>

<button onclick={sync}>go</button>
