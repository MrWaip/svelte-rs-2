<script>
	let shown = $state(true);
</script>

<svelte:options namespace="mathml" />

<annotation-xml>
	{#if shown}
		<div>fallback html</div>
	{/if}
</annotation-xml>
