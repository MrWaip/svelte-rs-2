<script>
	let { obj = $bindable() } = $props();
	function sync(cb) {
		cb(obj.field = obj.other);
	}
</script>

<button onclick={() => sync(() => {})}>go</button>
