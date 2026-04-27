<svelte:options runes={false} />
<script>
	import Inner from './Inner.svelte';
	export let value = '';
</script>

<Inner bind:value={value} />
