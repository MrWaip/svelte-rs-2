<script>
	let { component } = $props();
</script>
{#if component}
	{@const Component = component}
	<Component />
{/if}
