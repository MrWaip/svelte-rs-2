<script>
	let { Comp } = $props();
</script>

<svelte:component this={Comp} --my-var="baseline">
	{#snippet element({ idx })}
		<div>{idx}</div>
	{/snippet}
</svelte:component>
