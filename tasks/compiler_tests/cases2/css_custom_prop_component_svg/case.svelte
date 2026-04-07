<script>
	import Child from './Child.svelte';
	let color = $state('red');
</script>

<Child --color={color} />
