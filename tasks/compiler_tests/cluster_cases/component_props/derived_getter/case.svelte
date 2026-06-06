<script>
	import Child from './Child.svelte';
	let count = $state(0);
	count = 1;
	let doubled = $derived(count * 2);
</script>

<Child {doubled} />
