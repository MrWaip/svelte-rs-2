<script>
	import Child from './Child.svelte';

	await Promise.resolve();
	let b = true;
</script>

<Child {b} />
