<script>
	import Comp from './Comp.svelte';
	function getData() { return [1, 2, 3]; }
</script>

<Comp data={getData()} />