<svelte:options runes={false} />
<script>
	import Child from './Child.svelte';
	export let store;
</script>
<Child bind:value={$store.field} />
