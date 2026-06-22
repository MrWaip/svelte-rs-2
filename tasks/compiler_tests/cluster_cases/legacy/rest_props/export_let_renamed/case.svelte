<svelte:options runes={false} />
<script>
	let foo = 1;
	export { foo as bar };
</script>

<div {...$$restProps}>{foo}</div>
