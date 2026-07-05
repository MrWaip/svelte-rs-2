<script>
	import Child from './Child.svelte';
	let count = $state(0);
	count = 1;
</script>

<Child random={Math.max(count, 2)} />
