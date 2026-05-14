<script>
	import Child from './Child.svelte';
	let props = $props();
</script>

{#snippet body()}
	{#if props.Inner}
		<props.Inner />
	{/if}
{/snippet}

<Child icon={props.show ? body : undefined} />
