<script>
	import Child from './Child.svelte';
	function close() {}
	const typedClose = close;
</script>

<Child handler={typedClose} />
