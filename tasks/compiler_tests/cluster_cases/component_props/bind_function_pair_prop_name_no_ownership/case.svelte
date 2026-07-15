<svelte:options runes={false} />

<script>
	import Comp from './Comp.svelte';
	export let value;
</script>

<Comp bind:value={() => value, (v) => (value = v?.trim())} />
