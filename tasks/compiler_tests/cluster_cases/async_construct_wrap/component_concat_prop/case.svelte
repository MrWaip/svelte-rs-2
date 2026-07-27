<script>
	import Child from './Child.svelte';
	async function f() { return 1 }
</script>
<Child --c="1px" a="y{await f()}" />
