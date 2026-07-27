<script>
	let obj = $state({ x: null });
	let src = $state({});
	let depth = $state(0);
</script>

{#if depth}
	<svelte:self onChange={(v) => (obj.x = src)} depth={depth - 1} />
{/if}
