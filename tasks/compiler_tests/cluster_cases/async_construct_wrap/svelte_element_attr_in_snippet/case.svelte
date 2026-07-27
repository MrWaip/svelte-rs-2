<script>
	import Child from './Child.svelte';
	async function g() { return 2 }
</script>
<Child>
	{#snippet foo()}
		<svelte:element this={"div"} title={await g()} />
	{/snippet}
</Child>
