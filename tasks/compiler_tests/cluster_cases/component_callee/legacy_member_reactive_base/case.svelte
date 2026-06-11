<svelte:options runes={false} />
<script>
	export let lib = undefined;
</script>
{#if lib}
	{@const L = lib}
	<L.Button />
{/if}
