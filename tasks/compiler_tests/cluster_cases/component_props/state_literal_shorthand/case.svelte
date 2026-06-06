<script>
	import Child from './Child.svelte';
	let y = $state('y1');
</script>

<Child {y} />
