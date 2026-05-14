<svelte:options runes={false} />

<script>
	import Child from './Child.svelte';
	export let title;
</script>

<Child {title} />
