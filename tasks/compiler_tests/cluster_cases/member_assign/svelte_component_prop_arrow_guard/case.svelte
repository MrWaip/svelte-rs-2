<script>
	import Child from './Child.svelte';
	let obj = $state({ x: null });
	let src = $state({});
	let C = $state(Child);
</script>

<svelte:component this={C} onChange={(v) => (obj.x = src)} />
