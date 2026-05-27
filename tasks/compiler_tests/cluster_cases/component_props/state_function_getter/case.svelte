<script>
	import Child from './Child.svelte';
	let handler = $state((error) => console.error(error));
</script>

<Child {handler} />
