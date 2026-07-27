<script>
	import Child from './Child.svelte';
	async function f() { return 1 }
	async function g() { return 2 }
	let { tag } = $props();
</script>
<Child a={await f()}>
	<svelte:element this={tag} title={await g()} />
</Child>
