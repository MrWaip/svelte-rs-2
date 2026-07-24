<script>
	import Child from './Child.svelte';
	let obj = $state({ x: null });
	let src = $state({});
	let v = $state(0);
</script>

<Child bind:value={() => v, (n) => (obj.x = src)} />
