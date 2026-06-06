<script>
	import Child from './Child.svelte';
	const deferred = Promise.withResolvers();
</script>

<Child {deferred} />
