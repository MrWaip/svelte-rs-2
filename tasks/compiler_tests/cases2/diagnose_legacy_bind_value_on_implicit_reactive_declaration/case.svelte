<svelte:options runes={false} />

<script>
	import Child from './Child.svelte';
	export let store;
	$: value = $store;
	$: if (value !== $store) store.set(value);
</script>

<Child bind:value />
