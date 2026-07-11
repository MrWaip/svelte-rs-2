<script>
	import Child from './Child.svelte';
	let { store } = $props();
	const { data } = $derived(store);
</script>

<Child bind:value={$data.amount} />
