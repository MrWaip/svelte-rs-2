<script>
	import Child from './Child.svelte';
	function getX() { return 1; }
</script>

<Child random={getX()} />
