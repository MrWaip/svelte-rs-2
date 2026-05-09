<script>
	import Child from './Child.svelte';
	let color = $state('red');
</script>

<div>
	<Child --color={color} />
</div>
