<script>
	import Child from './Child.svelte';
	let value = $state();
</script>

<Child value={value} />
