<script>
	import Child from './Child.svelte';
	let count = $state(0);
</script>

{@debug count}
<Child {count} />
