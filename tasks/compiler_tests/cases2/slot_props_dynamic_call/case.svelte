<script>
	let item = "hello";

	function get_item() {
		return item;
	}
</script>

<slot item={get_item()} />
