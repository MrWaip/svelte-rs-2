<script>
	import Comp from './Comp.svelte';
	let { count } = $props();
	function getData() { return [1, 2, 3]; }
</script>

<Comp label="hello" data={getData()} count={count} />