<script>
	import Child from './Child.svelte';
	let color = $state('red');
	let columns = $state(3);
</script>

<Child --color={color} --columns={columns} />
