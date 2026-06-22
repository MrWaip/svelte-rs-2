<svelte:options runes={false} />
<script>
	export let component = undefined;
</script>
{#if component}
	{@const Component = component}
	<Component />
{/if}
