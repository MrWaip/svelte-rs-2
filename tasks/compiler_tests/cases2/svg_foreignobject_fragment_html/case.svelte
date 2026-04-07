<script>
	let shown = $state(true);
</script>

<svelte:options namespace="svg" />

<foreignObject>
	{#if shown}
		<div>fallback html</div>
	{/if}
</foreignObject>
