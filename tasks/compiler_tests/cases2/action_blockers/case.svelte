<script>
	let data = await fetch('/api');
	let config = $state(data.config);

	function tooltip(node, arg) {}
</script>

<div use:tooltip={config}>hello</div>
