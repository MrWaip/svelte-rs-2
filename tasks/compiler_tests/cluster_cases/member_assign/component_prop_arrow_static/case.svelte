<script>
	import Child from './Child.svelte';
	let obj = $state({ x: null });
	let src = $state({});
</script>

<Child onChange={(v) => (obj.x = src)} />
