<script>
	import Child from './Child.svelte';
	let items = $state([1, 2, 3]);
</script>

<Child {items} />
