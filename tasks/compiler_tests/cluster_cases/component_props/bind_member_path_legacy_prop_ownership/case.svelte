<svelte:options runes={false} />

<script>
	import Child from './Child.svelte';
	export let step;
</script>

<Child bind:value={step.editValue} />
