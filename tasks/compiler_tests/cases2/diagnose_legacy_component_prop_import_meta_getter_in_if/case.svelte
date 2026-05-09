<svelte:options runes={false} />
<script>
	import Inner from './Inner.svelte';
	export let cond;
</script>

{#if cond}
	<Inner url={import.meta.env.VITE_X} />
{/if}
