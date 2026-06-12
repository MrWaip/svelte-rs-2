<svelte:options runes={false} />
<script>
	const foo = 1;
	export { foo as bar };
</script>

<div {...$$restProps}>{foo}</div>
